use alloc::vec::Vec;

#[test]
pub fn no_alloc_error() {
    let mut test_vec: Vec<i32> = vec![];

    for _ in 0..1024 {
        test_vec.push(1);
    }

    assert_eq!(1024, test_vec.len());
}