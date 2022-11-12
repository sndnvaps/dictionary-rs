use dictionary::Dictionary;
fn main() {
    let mut _d: Dictionary<u8, u8> = Dictionary::new();
    _d.insert(1, 2);
    _d.insert(3, 4);

    println!("{}", *_d.get(&3).unwrap());
}
