use hex::hex;

#[test]
fn test_valid_hex_array() {
    const DATA: [u8; 11] = hex!("48656c6c6f20776f726c64");
    assert_eq!(DATA, *b"Hello world");
}

//#[test]
//#[should_panic(expected = "Invalid hex string")]
//fn test_invalid_hex() {
//    const DATA: [u8; 11] = hex!("48656c6c6f20776f726c6g");
//}
