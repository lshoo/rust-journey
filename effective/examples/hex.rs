
fn main() {
    use hex_literal::hex;

    // The macro can be used in const contexts
    const DATA: [u8; 4] = hex!("01020304");
    assert_eq!(DATA, [1, 2, 3, 4]);

    // Both upper and lower hex values are supported
    assert_eq!(hex!("a1 b2 c3 d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
    assert_eq!(hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
    assert_eq!(hex!("0a0B 0C0d"), [10, 11, 12, 13]);

    // Multi-line literals
    let bytes1 = hex!("
        00010203 04050607
        08090a0b 0c0d0e0f
    ");
    assert_eq!(
        bytes1,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    );

    // It's possible to use several literals
    // (results will be concatenated)
    let bytes2 = hex!(
        "00010203 04050607" // first half
        "08090a0b 0c0d0e0f" // second half
    );
    assert_eq!(bytes1, bytes2);

    println!("Ok")
}