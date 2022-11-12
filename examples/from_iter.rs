use dictionary::Dictionary;

fn main() {
    let tuples: Vec<(u8, u8)> = vec![(1, 3), (5, 7)];
    let _d: Dictionary<u8, u8> = Dictionary::from_iter(tuples.iter());
    let tuples_array: Vec<(u8, [u8; 5])> = vec![(2, [3, 4, 5, 6, 7]), (8, [9, 10, 11, 12, 13])];
    let _d_array: Dictionary<u8, [u8; 5]> = Dictionary::from_iter(tuples_array.iter());

    println!("{}", *_d.get(&5).unwrap());
    println!("{:?}", *_d_array.get(&8).unwrap());
}
