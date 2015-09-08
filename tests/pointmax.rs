extern crate interval_tree;

use interval_tree::pointmax::{PointMaxTree};

#[test]
fn queries_on_empty() {
    let t = PointMaxTree::new(0, 10);

    assert_eq!(t.query(-1), None);
    assert_eq!(t.query(11), None);
    assert_eq!(t.query(1), Some(0));
    assert_eq!(t.query(5), Some(0));
}

#[test]
#[should_panic]
fn invalid_tree() {
    let _ = PointMaxTree::new(0, -1);
}

#[test]
#[should_panic]
fn invalid_insert() {
    let mut t = PointMaxTree::new(0, 10);
    t.insert(5, 15, 1);
}

#[test]
fn small_queries() {
    let mut t = PointMaxTree::new(0, 10);
    t.insert(0, 2, 1);
    t.insert(0, 5, 10);
    t.insert(5, 6, 2);
    t.insert(5, 7, 1);
    t.insert(10, 10, 5);
    println!("{:?}", t);
    let values = vec![11, 11, 11, 10, 10, 13, 3, 1, 0, 0, 5];
    assert_eq!(t.query(-1), None);
    assert_eq!(t.query(11), None);
    assert_eq!(t.query(100), None);
    for (p, v) in (0..10).zip(values) {
        println!("{}", p);
        assert_eq!(t.query(p), Some(v));
    }
}
