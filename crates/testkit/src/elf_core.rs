//! Small Linux core notes for register and crash-evidence tests.

fn put16(bytes: &mut [u8], at: usize, value: u16, big: bool) {
    bytes[at..at + 2].copy_from_slice(&if big { value.to_be_bytes() } else { value.to_le_bytes() });
}

fn put32(bytes: &mut [u8], at: usize, value: u32, big: bool) {
    bytes[at..at + 4].copy_from_slice(&if big { value.to_be_bytes() } else { value.to_le_bytes() });
}

fn note(notes: &mut Vec<u8>, kind: u32, desc: &[u8], big: bool) {
    let start = notes.len();
    notes.resize(start + 20, 0);
    put32(notes, start, 5, big);
    put32(notes, start + 4, desc.len() as u32, big);
    put32(notes, start + 8, kind, big);
    notes[start + 12..start + 17].copy_from_slice(b"CORE\0");
    notes.extend_from_slice(desc);
    while notes.len() % 4 != 0 {
        notes.push(0);
    }
}

/// One ARM32 thread with the Linux r0-r15, CPSR and original-r0 register layout.
/// The file mapping covers 0x10000..0x11000; no process memory is included.
pub fn arm32(registers: &[u32; 18], big: bool) -> Vec<u8> {
    let mut status = vec![0; 148];
    put32(&mut status, 0, 4, big); // SIGILL
    put16(&mut status, 12, 4, big);
    put32(&mut status, 24, 4242, big);
    for (index, value) in registers.iter().enumerate() {
        put32(&mut status, 72 + index * 4, *value, big);
    }
    let mut notes = Vec::new();
    note(&mut notes, 1, &status, big);
    let mut mapping = vec![0; 20];
    for (index, value) in [1, 4096, 0x10000, 0x11000, 0].into_iter().enumerate() {
        put32(&mut mapping, index * 4, value, big);
    }
    mapping.extend_from_slice(b"/usr/bin/arm-fixture\0");
    note(&mut notes, 0x4649_4c45, &mapping, big);
    let mut bytes = vec![0; 84];
    bytes[..7].copy_from_slice(&[0x7f, b'E', b'L', b'F', 1, if big { 2 } else { 1 }, 1]);
    put16(&mut bytes, 16, 4, big);
    put16(&mut bytes, 18, 40, big);
    put32(&mut bytes, 20, 1, big);
    put32(&mut bytes, 28, 52, big);
    put16(&mut bytes, 40, 52, big);
    put16(&mut bytes, 42, 32, big);
    put16(&mut bytes, 44, 1, big);
    put32(&mut bytes, 52, 4, big);
    put32(&mut bytes, 56, 84, big);
    put32(&mut bytes, 68, notes.len() as u32, big);
    put32(&mut bytes, 72, notes.len() as u32, big);
    put32(&mut bytes, 80, 4, big);
    bytes.extend_from_slice(&notes);
    bytes
}
