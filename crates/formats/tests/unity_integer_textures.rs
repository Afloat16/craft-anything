use craft_anything_formats::unity_texture;

#[test]
fn r16_uses_little_endian_unsigned_samples_and_the_full_range() {
    for (sample, grey) in [(0u16, 0), (128, 0), (129, 1), (0x1234, 18), (32768, 128), (u16::MAX, 255)] {
        assert_eq!(unity_texture::mip0_size(9, 1, 1), Some(2));
        assert_eq!(unity_texture::decode(9, 1, 1, &sample.to_le_bytes()).unwrap(), [grey, grey, grey, 255]);
    }
}

#[test]
fn rg16_keeps_two_eight_bit_channels_without_inventing_blue() {
    assert_eq!(unity_texture::mip0_size(62, 2, 1), Some(4));
    assert_eq!(unity_texture::decode(62, 2, 1, &[10, 200, 255, 0]).unwrap(), [10, 200, 0, 255, 255, 0, 0, 255]);
}

