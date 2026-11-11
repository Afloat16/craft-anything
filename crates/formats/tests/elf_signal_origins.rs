use craft_anything_formats::elf::{self, SigInfo};

#[derive(Clone, Copy)]
struct Layout {
    is64: bool,
    big: bool,
}

const LAYOUTS: [Layout; 4] = [
    Layout { is64: false, big: false },
    Layout { is64: false, big: true },
    Layout { is64: true, big: false },
    Layout { is64: true, big: true },
];
const ADDRESS: u64 = 0x1234_abcd;

impl Layout {
    fn put16(self, bytes: &mut [u8], at: usize, value: u16) {
        bytes[at..at + 2].copy_from_slice(&if self.big { value.to_be_bytes() } else { value.to_le_bytes() });
    }

    fn put32(self, bytes: &mut [u8], at: usize, value: u32) {
        bytes[at..at + 4].copy_from_slice(&if self.big { value.to_be_bytes() } else { value.to_le_bytes() });
    }

    fn word(self, bytes: &mut [u8], at: usize, value: u64) {
        if self.is64 {
            bytes[at..at + 8].copy_from_slice(&if self.big { value.to_be_bytes() } else { value.to_le_bytes() });
        } else {
            self.put32(bytes, at, u32::try_from(value).unwrap());
        }
    }

    fn signal(self, signo: i32, code: i32) -> Vec<u8> {
        let mut desc = vec![0; 128];
        self.put32(&mut desc, 0, signo as u32);
        self.put32(&mut desc, 4, 17);
        self.put32(&mut desc, 8, code as u32);
        // These bytes are deliberately nonzero. In sender/timer variants they
        // are other fields of the union, so their apparent address is not evidence.
        self.word(&mut desc, if self.is64 { 16 } else { 12 }, ADDRESS);
        desc
    }

    fn status(self, signo: u16) -> Vec<u8> {
        let mut desc = vec![0; if self.is64 { 384 } else { 144 }];
        self.put16(&mut desc, 12, signo);
        self.put32(&mut desc, if self.is64 { 32 } else { 24 }, 4242);
        desc
    }

    fn core(self, notes: &[(u32, Vec<u8>)]) -> Vec<u8> {
        let mut payload = Vec::new();
        for (kind, desc) in notes {
            let start = payload.len();
            payload.resize(start + 20, 0);
            self.put32(&mut payload, start, 5);
            self.put32(&mut payload, start + 4, desc.len() as u32);
            self.put32(&mut payload, start + 8, *kind);
            payload[start + 12..start + 17].copy_from_slice(b"CORE\0");
            payload.extend_from_slice(desc);
            payload.resize((payload.len() + 3) & !3, 0);
        }
        let header = if self.is64 { 64 } else { 52 };
        let phsize = if self.is64 { 56 } else { 32 };
        let offset = header + phsize;
        let mut bytes = vec![0; offset];
        bytes[..4].copy_from_slice(b"\x7fELF");
        bytes[4] = if self.is64 { 2 } else { 1 };
        bytes[5] = if self.big { 2 } else { 1 };
        bytes[6] = 1;
        self.put16(&mut bytes, 16, 4);
        self.put16(&mut bytes, 18, if self.is64 { 62 } else { 3 });
        self.put32(&mut bytes, 20, 1);
        self.word(&mut bytes, if self.is64 { 32 } else { 28 }, header as u64);
        let ehsize = if self.is64 { 52 } else { 40 };
        self.put16(&mut bytes, ehsize, header as u16);
        self.put16(&mut bytes, ehsize + 2, phsize as u16);
        self.put16(&mut bytes, ehsize + 4, 1);
        self.put32(&mut bytes, header, 4);
        self.word(&mut bytes, header + if self.is64 { 8 } else { 4 }, offset as u64);
        self.word(&mut bytes, header + if self.is64 { 32 } else { 16 }, payload.len() as u64);
        self.word(&mut bytes, header + if self.is64 { 40 } else { 20 }, payload.len() as u64);
        bytes.extend(payload);
        bytes
    }

    fn parsed(self, signo: i32, code: i32) -> SigInfo {
        elf::parse_core(&self.core(&[(0x5349_4749, self.signal(signo, code))])).unwrap().signal.unwrap()
    }
}

#[test]
fn process_thread_and_queued_signals_do_not_publish_sender_bytes_as_fault_addresses() {
    for layout in LAYOUTS {
        for signo in [4, 6, 7, 8, 11] {
            for (code, origin) in [(0, "Sent by a process"), (-6, "Sent to a thread"), (-1, "Queued by a process")] {
                let signal = layout.parsed(signo, code);
                assert_eq!((signal.signo, signal.code, signal.errno), (signo, code, 17));
                assert!(signal.fault_addr.is_none(), "{signal:?}");
                assert!(signal.meaning.contains(origin), "{signal:?}");
            }
        }
    }
}

#[test]
fn timer_and_asynchronous_notifications_do_not_publish_their_union_as_an_address() {
    for layout in LAYOUTS {
        for (code, origin) in [(-2, "timer"), (-3, "message queue"), (-4, "asynchronous I/O"), (-5, "queued I/O"), (-7, "execve"), (-60, "name lookup")] {
            let signal = layout.parsed(11, code);
            assert!(signal.fault_addr.is_none(), "{signal:?}");
            assert!(signal.meaning.contains(origin), "{signal:?}");
        }
        let unknown = layout.parsed(11, -123);
        assert!(unknown.fault_addr.is_none());
        assert!(unknown.meaning.contains("si_code -123"));
    }
}

#[test]
fn kernel_signals_without_a_fault_code_do_not_interpret_sender_storage() {
    for layout in LAYOUTS {
        for signo in [4, 5, 7, 8, 11] {
            let signal = layout.parsed(signo, 128);
            assert!(signal.fault_addr.is_none(), "{signal:?}");
            assert!(signal.meaning.contains("Sent by the kernel"), "{signal:?}");
        }
    }
}

#[test]
fn unknown_positive_codes_and_ptrace_event_codes_do_not_invent_fault_addresses() {
    for layout in LAYOUTS {
        for (signo, code) in [(4, 12), (5, 7), (7, 6), (8, 16), (11, 11), (11, 129), (5, (1 << 8) | 5)] {
            let signal = layout.parsed(signo, code);
            assert!(signal.fault_addr.is_none(), "{signal:?}");
            assert!(signal.meaning.contains(&format!("si_code {code}")), "{signal:?}");
        }
    }
}

#[test]
fn actual_fault_variants_preserve_addresses_in_each_class_and_byte_order() {
    for layout in LAYOUTS {
        for (signo, code) in [(4, 1), (4, 11), (7, 1), (7, 5), (8, 1), (8, 15), (11, 1), (11, 10)] {
            let signal = layout.parsed(signo, code);
            assert_eq!(signal.fault_addr, Some(ADDRESS), "{signal:?}");
            assert_eq!((signal.signo, signal.code, signal.errno), (signo, code, 17));
        }
    }
}

#[test]
fn signal_specific_traps_keep_the_address_that_their_variant_records() {
    for layout in LAYOUTS {
        for code in 1..=6 {
            let signal = layout.parsed(5, code);
            assert_eq!(signal.fault_addr, Some(ADDRESS), "{signal:?}");
            assert!(signal.meaning.contains("breakpoint or trap"));
        }
    }
}

#[test]
fn a_prstatus_signal_without_siginfo_keeps_its_origin_unknown() {
    for layout in LAYOUTS {
        for signo in [4, 6, 11] {
            let core = elf::parse_core(&layout.core(&[(1, layout.status(signo))])).unwrap();
            assert_eq!(core.threads[0].pid, 4242);
            let signal = core.signal.unwrap();
            assert_eq!(signal.signo, i32::from(signo));
            assert!(signal.fault_addr.is_none());
            assert!(signal.meaning.contains("origin was not recorded"), "{signal:?}");
        }
    }
}

#[test]
fn a_truncated_siginfo_header_does_not_invent_a_zero_code_or_replace_prstatus() {
    for layout in LAYOUTS {
        let desc = layout.signal(11, -6);
        for end in 0..12 {
            let short = desc[..end].to_vec();
            let core = elf::parse_core(&layout.core(&[(1, layout.status(4)), (0x5349_4749, short.clone())])).unwrap();
            let signal = core.signal.unwrap();
            assert_eq!(signal.signo, 4);
            assert!(signal.meaning.contains("origin was not recorded"), "{signal:?}");
            assert!(core.warnings.iter().any(|w| w.contains("NT_SIGINFO header is truncated")));
            let only = elf::parse_core(&layout.core(&[(0x5349_4749, short)])).unwrap();
            assert!(only.signal.is_none());
            assert!(only.warnings.iter().any(|w| w.contains("NT_SIGINFO header is truncated")));
        }
    }
}

#[test]
fn a_complete_signal_header_does_not_invent_missing_address_bytes() {
    for layout in LAYOUTS {
        let desc = layout.signal(11, 1);
        let address_end = if layout.is64 { 24 } else { 16 };
        for end in 12..address_end {
            let core = elf::parse_core(&layout.core(&[(0x5349_4749, desc[..end].to_vec())])).unwrap();
            let signal = core.signal.unwrap();
            assert_eq!((signal.signo, signal.code), (11, 1));
            assert!(signal.fault_addr.is_none());
            assert!(signal.meaning.contains("isn't mapped"));
        }
    }
}

#[test]
fn existing_specific_hardware_fault_explanations_remain_precise() {
    for layout in LAYOUTS {
        for (signo, code, explanation) in [(11, 1, "isn't mapped"), (11, 2, "wrong permissions"), (8, 1, "division by zero")] {
            let signal = layout.parsed(signo, code);
            assert!(signal.meaning.contains(explanation), "{signal:?}");
            assert_eq!(signal.fault_addr, Some(ADDRESS));
        }
    }
}
