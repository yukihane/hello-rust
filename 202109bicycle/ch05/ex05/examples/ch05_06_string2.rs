fn main() {
    let bad_utf8: [u8; 7] = [b'a', 0xf0, 0x90, 0x80, 0xe3, 0x81, 0x82];
    let s = String::from_utf8_lossy(&bad_utf8);
    println!("{}", s);
    let rabbit = "rabbit".to_string();
    let r: &str = &rabbit;
    println!("{}", rabbit);
    let r = 0..;
    let e = ..10;
    let r2 = 0..10;
}
