extern crate interval_tree;

use interval_tree::segmentpoint::{SegmentPointTree};

#[test]
fn queries_on_empty() {
    let t = SegmentPointTree::new(0, 10, 0, Box::new(|x: &i64, y: &i64| x + y));

    assert_eq!(t.query(-1), None);
    assert_eq!(t.query(11), None);
    assert_eq!(t.query(1), Some(0));
    assert_eq!(t.query(5), Some(0));
}

#[test]
#[should_panic]
fn invalid_tree() {
    let _ = SegmentPointTree::new(1000, 10, 0, Box::new(|x: &i64, y: &i64| x + y));
}

#[test]
#[should_panic]
fn invalid_insert() {
    let mut t = SegmentPointTree::new(0, 10, 0, Box::new(|x: &i64, y: &i64| x + y));
    t.insert(5, 15, 1);
}

#[test]
fn singleton_tree() {
    let mut t = SegmentPointTree::new(0, 0, 0, Box::new(|x: &i64, y: &i64| x + y));
    t.insert(0, 0, 10);
    assert_eq!(t.query(0), Some(10));
    t.insert(0, 0, 11);
    assert_eq!(t.query(0), Some(21));
    assert_eq!(t.query(1), None);
    assert_eq!(t.query(-11), None);
    assert_eq!(t.bounds(), (0, 0));
}

#[test]
fn small_queries() {
    let mut t = SegmentPointTree::new(0, 10, 0, Box::new(|x: &i32, y: &i32| x + y));
    t.insert(0, 2, 1);
    t.insert(0, 5, 10);
    t.insert(5, 6, 2);
    t.insert(5, 7, 1);
    t.insert(10, 10, 5);
    assert_eq!(t.bounds(), (0, 10));
    let values = vec![11, 11, 11, 10, 10, 13, 3, 1, 0, 0, 5];
    assert_eq!(t.query(-1), None);
    assert_eq!(t.query(11), None);
    assert_eq!(t.query(100), None);
    for (p, v) in (0..10).zip(values) {
        assert_eq!(t.query(p), Some(v));
    }
}

#[test]
fn small_queries_2() {
    let mut t = SegmentPointTree::new(0, 10, 1, Box::new(|x: &i32, y: &i32| x * y));
    t.insert(0, 2, 1);
    t.insert(0, 5, 3);
    t.insert(5, 6, 2);
    t.insert(5, 7, 1);
    t.insert(10, 10, 0);
    assert_eq!(t.bounds(), (0, 10));
    let values = vec![3, 3, 3, 3, 3, 6, 2, 1, 1, 1, 0];
    assert_eq!(t.query(-1), None);
    assert_eq!(t.query(11), None);
    assert_eq!(t.query(100), None);
    for (p, v) in (0..10).zip(values) {
        assert_eq!(t.query(p), Some(v));
    }
}

#[test]
fn small_queries_3() {
    let concat_f = Box::new(|x: &String, y: &String| (x.clone() + &*y).to_string());
    let mut t:SegmentPointTree<i64, String> = SegmentPointTree::new(4, 6, "".to_string(), concat_f);
    assert_eq!(t.bounds(), (4, 6));
    t.insert(4, 5, "a".to_string());
    t.insert(5, 6, "b".to_string());
    t.insert(5, 5, "c".to_string());
    t.insert(6, 6, "x".to_string());
    let values = vec!["a".to_string(), "abc".to_string(), "bx".to_string()];
    assert_eq!(t.query(3), None);
    assert_eq!(t.query(11), None);
    for (p, v) in (4..6).zip(values) {
        assert_eq!(t.query(p), Some(v));
    }
}

