use craft_anything_formats::{binary::Arch, elf};
use craft_anything_testkit::elf_core::arm32;

const NAMES: [&str; 18] = ["r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "sp", "lr", "pc", "cpsr", "orig_r0"];

fn values() -> [u32; 18] {
    let mut values = std::array::from_fn(|index| 0x8000_1000 + index as u32 * 0x101);
    values[15] = 0x10234;
    values[16] = 0x10;
    values
}

fn check_complete(big: bool) {
    let expected = values();
    let core = elf::parse_core(&arm32(&expected, big)).unwrap();
    assert_eq!(core.arch, Arch::Arm);
    assert_eq!(core.bits, 32);
    assert_eq!(core.threads.len(), 1);
    assert_eq!(core.threads[0].pid, 4242);
    assert_eq!(core.threads[0].signal, 4);
    let registers: Vec<_> = core.threads[0].registers.iter().map(|reg| (reg.name, reg.value)).collect();
    assert_eq!(registers, NAMES.into_iter().zip(expected.into_iter().map(u64::from)).collect::<Vec<_>>());
    assert_eq!(core.pc(), Some(0x10234));
    let (mapping, offset) = core.locate(core.pc().unwrap()).unwrap();
    assert_eq!(mapping.path, "/usr/bin/arm-fixture");
    assert_eq!(offset, 0x234);
}

#[test]
fn linux_arm_registers_decode_in_little_endian_cores() {
    check_complete(false);
}

#[test]
fn linux_arm_registers_decode_in_big_endian_cores() {
    check_complete(true);
}

#[test]
fn thumb_state_is_preserved_without_changing_the_recorded_pc() {
    for big in [false, true] {
        let mut expected = values();
        expected[15] = 0x10232;
        expected[16] = 0x30 | if big { 0x200 } else { 0 };
        let core = elf::parse_core(&arm32(&expected, big)).unwrap();
        assert_eq!(core.pc(), Some(0x10232));
        assert_eq!(core.threads[0].registers.iter().find(|reg| reg.name == "cpsr").unwrap().value, u64::from(expected[16]));
        assert_eq!(core.threads[0].registers.iter().find(|reg| reg.name == "orig_r0").unwrap().value, u64::from(expected[17]));
    }
}

#[test]
fn truncated_register_words_are_not_invented_or_read_from_the_next_note() {
    for big in [false, true] {
        let expected = values();
        let original = arm32(&expected, big);
        // The first note has a 20-byte header/name followed by elf_prstatus.
        for size in 72..148 {
            let mut bytes = original[..104 + size].to_vec();
            let put = |bytes: &mut [u8], at: usize, value: u32| {
                bytes[at..at + 4].copy_from_slice(&if big { value.to_be_bytes() } else { value.to_le_bytes() });
            };
            put(&mut bytes, 88, size as u32);
            put(&mut bytes, 68, (20 + size) as u32);
            put(&mut bytes, 72, (20 + size) as u32);
            let core = elf::parse_core(&bytes).unwrap();
            let count = ((size - 72) / 4).min(18);
            assert_eq!(core.threads[0].registers.len(), count, "{big} {size}");
            for (index, register) in core.threads[0].registers.iter().enumerate() {
                assert_eq!((register.name, register.value), (NAMES[index], u64::from(expected[index])));
            }
            assert_eq!(core.pc(), (count > 15).then_some(0x10234));
        }
    }
}

#[test]
fn a_second_thread_keeps_its_own_registers_and_does_not_replace_the_crashing_pc() {
    let expected = values();
    let mut bytes = arm32(&expected, false);
    let mut second = bytes[84..84 + 20 + 148].to_vec();
    second[20 + 24..20 + 28].copy_from_slice(&4243u32.to_le_bytes());
    second[20 + 72 + 15 * 4..20 + 72 + 16 * 4].copy_from_slice(&0x10568u32.to_le_bytes());
    bytes.extend_from_slice(&second);
    let length = (bytes.len() - 84) as u32;
    bytes[68..72].copy_from_slice(&length.to_le_bytes());
    bytes[72..76].copy_from_slice(&length.to_le_bytes());
    let core = elf::parse_core(&bytes).unwrap();
    assert_eq!(core.threads.len(), 2);
    assert_eq!(core.pc(), Some(0x10234));
    assert_eq!(core.threads[1].pid, 4243);
    assert_eq!(core.threads[1].registers.iter().find(|reg| reg.name == "pc").unwrap().value, 0x10568);
}

#[test]
fn an_unknown_machine_does_not_receive_arm_register_names() {
    let mut bytes = arm32(&values(), false);
    bytes[18..20].copy_from_slice(&243u16.to_le_bytes());
    let core = elf::parse_core(&bytes).unwrap();
    assert_eq!(core.arch, Arch::Other);
    assert!(core.threads[0].registers.is_empty());
    assert_eq!(core.pc(), None);
}

#[test]
fn an_invalid_64_bit_arm_machine_does_not_use_the_32_bit_layout() {
    let mut bytes = craft_anything_testkit::elf64_core(0x400234, 0x400234);
    bytes[18..20].copy_from_slice(&40u16.to_le_bytes());
    let core = elf::parse_core(&bytes).unwrap();
    assert_eq!(core.bits, 64);
    assert!(core.threads[0].registers.is_empty());
    assert_eq!(core.pc(), None);
}

#[test]
fn x86_64_register_names_and_pc_are_unchanged() {
    let core = elf::parse_core(&craft_anything_testkit::elf64_core(0x400234, 0x400234)).unwrap();
    assert_eq!(core.arch, Arch::X86_64);
    assert_eq!(core.threads[0].registers.len(), 27);
    assert_eq!(core.pc(), Some(0x400234));
    assert!(core.threads[0].registers.iter().any(|reg| reg.name == "rsp" && reg.value == 0x7ffd_0000_1000));
}
