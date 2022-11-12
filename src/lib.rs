extern crate indexmap;

pub mod dictionary;

pub use crate::dictionary::Dictionary;

/*
 * Creates the dictionary
 * {
 * 1: 6,
 * 2: 7,
 * 3: 8,
 * 4: 9,
 * 5: 0
 * }
 */
#[allow(dead_code)]
fn create_dict() -> Dictionary<u8, u8> {
    let tuples: Vec<(u8, u8)> = vec![(1, 6), (2, 7), (3, 8), (4, 9), (5, 0)];
    Dictionary::from_tuples(tuples)
}

#[allow(dead_code)]
fn has_same_elements<T: PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    for i in vec1 {
        if vec2.contains(i) {
            continue;
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn zero_sized_dict() {
        let _d: Dictionary<u8, u8> = Dictionary::with_capacity(0);
    }

    #[test]
    fn create_from_vecs() {
        let vec1: Vec<usize> = vec![1, 2, 3, 4, 5];
        let vec2: Vec<usize> = vec![6, 7, 8, 9, 0];
        let _d: Dictionary<usize, usize> = Dictionary::from_vecs(vec1, vec2);

        assert_eq!(_d.len(), 5);
    }

    #[test]
    fn create_from_tuples() {
        let tuples: Vec<(i8, i8)> = vec![(1, 2), (3, 4)];
        let _d: Dictionary<i8, i8> = Dictionary::from_tuples(tuples);

        assert_eq!(*_d.get(&1).unwrap(), 2);
    }

    #[test]
    fn create_from_iter() {
        let tuples: Vec<(u8, u8)> = vec![(1, 3), (5, 7)];
        let _d: Dictionary<u8, u8> = Dictionary::from_iter(tuples.iter());
        let tuples_array: Vec<(u8, [u8; 5])> = vec![(2, [3, 4, 5, 6, 7]), (8, [9, 10, 11, 12, 13])];
        let _d_array: Dictionary<u8, [u8; 5]> = Dictionary::from_iter(tuples_array.iter());

        assert_eq!(*_d.get(&5).unwrap(), 7);
        assert_eq!(*_d_array.get(&8).unwrap(), [9, 10, 11, 12, 13]);
    }

    #[test]
    fn iterator_next() {
        let tuples: Vec<(u8, u8)> = vec![(1, 3), (5, 7), (9, 11), (13, 15)];
        let _d: Dictionary<u8, u8> = Dictionary::from_iter(tuples.iter());
        println!("iterator_next -> _d = {:?}", _d);

        let mut _d_iter = _d.iter();

        //println!("iterator_next -> _d_iter = {:?}", _d_iter);
        let _d_iter_val = _d_iter.next().unwrap();
        //println!("iterator_next test -> {:?}", _d_iter_val);
        let _d_iter_val = _d_iter.next().unwrap();
        assert_eq!(_d_iter_val, (&5, &7));

        //println!("iterator_next test -> {:?}", _d_iter_val);
    }

    #[test]
    #[should_panic]
    fn zero_sized_tuple_dict() {
        let tuples: Vec<(u8, u8)> = Vec::new();
        let _d: Dictionary<u8, u8> = Dictionary::from_tuples(tuples);
    }

    #[test]
    #[should_panic]
    fn paniced_from_vecs() {
        let vec1: Vec<usize> = vec![1, 2, 3, 4];
        let vec2: Vec<usize> = vec![5, 6, 7];
        let _d = Dictionary::from_vecs(vec1, vec2);
    }

    #[test]
    #[should_panic]
    fn zero_sized_vecs() {
        let vec1: Vec<u8> = Vec::new();
        let vec2: Vec<u8> = Vec::new();
        let _d = Dictionary::from_vecs(vec1, vec2);
    }

    #[test]
    fn lookup() {
        let _d = create_dict();

        assert_eq!(*_d.get(&1).unwrap(), 6);
    }

    #[test]
    fn insert() {
        let mut _d: Dictionary<u8, u8> = Dictionary::new();
        _d.insert(1, 2);

        assert_eq!(*_d.get(&1).unwrap(), 2);
    }

    #[test]
    fn size() {
        let _d = create_dict();
        assert_eq!(_d.len(), 5);
    }

    #[test]
    fn contains() {
        let mut _d = create_dict();

        assert!(_d.contains_key(&1));
    }

    #[test]
    fn remove() {
        let mut _d = create_dict();
        let _r = _d.remove(&1);

        assert!((!_d.contains_key(&1)) && _r.is_some() && _r.unwrap() == (6) && _d.len() == 4);
    }

    #[test]
    fn down_size() {
        let mut _d = create_dict();

        _d.remove(&1);
        _d.remove(&2);

        assert_eq!(_d.len(), 3);
    }

    #[test]
    fn remove_panic() {
        let mut _d: Dictionary<u8, u8> = Dictionary::new();
        _d.remove(&1);
    }
}
