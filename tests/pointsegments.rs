extern crate interval_tree;

use interval_tree::pointsegment::{PointSegmentTree};
use std::cmp::{max};

#[test]
fn queries_on_empty() { 
    let t = PointSegmentTree::new(0, 10, 0,
                                  Box::new(|x: &i64, y: &i64| x + y));
    assert_eq!(t.query(0, 10), Some(0));
    assert_eq!(t.query(5, 10), Some(0));
    assert_eq!(t.query(5, 5), Some(0));
    assert_eq!(t.query(10, 0), None);
    assert_eq!(t.query(-1, 5), None);
    assert_eq!(t.query(5, 11), None);
}

#[test]
#[should_panic]
fn invalid_tree() {
    let _ = PointSegmentTree::new(100, 10, 0,
                                  Box::new(|x: &i64, y: &i64| x + y));
}

#[test]
#[should_panic]
fn invalid_insert_upper() {
    let mut t = PointSegmentTree::new(0, 10, 0,
                                      Box::new(|x: &i64, y: &i64| x + y));

    t.insert(200, 0);
}

#[test]
#[should_panic]
fn invalid_insert_lower() {
    let mut t = PointSegmentTree::new(0, 10, 0,
                                      Box::new(|x: &i64, y: &i64| x + y));

    t.insert(-200, 0);
}


#[test]
fn singleton_tree() {
    let mut t = PointSegmentTree::new(5, 5, 0,
                                      Box::new(|x: &i64, y: &i64| x + y));
    assert_eq!(t.query(5, 5), Some(0));
    t.insert(5, 1);
    assert_eq!(t.query(5, 5), Some(1));
    t.insert(5, 100);
    assert_eq!(t.query(5, 5), Some(100));
}

#[test]
fn test_bounds() {
    let t = PointSegmentTree::new(3, 15, 0,
                                      Box::new(|x: &i64, y: &i64| max(*x, *y)));
    assert_eq!(t.bounds(), (3, 15));
}

#[test]
fn small_queries() {
    let mut t = PointSegmentTree::new(3, 15, 0,
                                      Box::new(|x: &i64, y: &i64| max(*x, *y)));
    t.insert(3, 1);
    t.insert(4, 2);
    t.insert(5, 5);
    assert_eq!(t.query(3, 3), Some(1));
    assert_eq!(t.query(4, 4), Some(2));
    assert_eq!(t.query(5, 5), Some(5));
    assert_eq!(t.query(3, 5), Some(5));
    assert_eq!(t.query(6, 10), Some(0));
    assert_eq!(t.query(5, 10), Some(5));
    assert_eq!(t.query(-10, 10), None);
    assert_eq!(t.query(100, 200), None);
    t.insert(6, 7);
    t.insert(9, 5);
    t.insert(11, 7);
    t.insert(12, 10);
    t.insert(14, 2);
    assert_eq!(t.query(6, 6), Some(7));
    assert_eq!(t.query(7, 7), Some(0));
    assert_eq!(t.query(9, 9), Some(5));
    assert_eq!(t.query(11, 11), Some(7));
    assert_eq!(t.query(12, 12), Some(10));
    assert_eq!(t.query(13, 13), Some(0));
    assert_eq!(t.query(14, 14), Some(2));
    assert_eq!(t.query(6, 9), Some(7));
    assert_eq!(t.query(7, 9), Some(5));
    assert_eq!(t.query(11, 13), Some(10));
    assert_eq!(t.query(12, 15), Some(10));
    assert_eq!(t.query(13, 15), Some(2));
    assert_eq!(t.query(14, 15), Some(2));
    assert_eq!(t.query(15, 15), Some(0));
}

#[test]
fn small_queries_2() {
    let mut t = PointSegmentTree::new(2, 15, 1.0,
                                      Box::new(|x: &f64, y: &f64| x * y));
    t.insert(3, 10.);
    t.insert(2, -1.);
    t.insert(5, 5.);
    assert_eq!(t.query(3, 3), Some(10.));
    assert_eq!(t.query(2, 2), Some(-1.));
    assert_eq!(t.query(5, 5), Some(5.));
    assert_eq!(t.query(3, 5), Some(50.));
    assert_eq!(t.query(6, 10), Some(1.));
    assert_eq!(t.query(2, 10), Some(-50.));
    assert_eq!(t.query(-10, 10), None);
    assert_eq!(t.query(100, 200), None);
    t.insert(6, 7.);
    t.insert(9, 5.);
    t.insert(11, 7.);
    t.insert(12, 10.);
    t.insert(14, 2.);
    assert_eq!(t.query(6, 6), Some(7.));
    assert_eq!(t.query(7, 7), Some(1.));
    assert_eq!(t.query(9, 9), Some(5.));
    assert_eq!(t.query(11, 11), Some(7.));
    assert_eq!(t.query(12, 12), Some(10.));
    assert_eq!(t.query(13, 13), Some(1.));
    assert_eq!(t.query(14, 14), Some(2.));
    assert_eq!(t.query(6, 9), Some(35.));
    assert_eq!(t.query(7, 9), Some(5.));
    assert_eq!(t.query(11, 13), Some(70.));
    assert_eq!(t.query(12, 15), Some(20.));
    assert_eq!(t.query(13, 15), Some(2.));
    assert_eq!(t.query(14, 15), Some(2.));
    assert_eq!(t.query(15, 15), Some(1.));
    assert_eq!(t.query(-10, 10), None);
    assert_eq!(t.query(100, 200), None);
}

#[test]
fn large_queries() {
    let mut t = PointSegmentTree::new(10, 1_000_000, 0,
                                      Box::new(|x: &i64, y: &i64| x + y));
    t.insert(100, 1);
    t.insert(200, 1);
    t.insert(300, 1);
    assert_eq!(t.query(10, 150), Some(1));
    assert_eq!(t.query(150, 250), Some(1));
    assert_eq!(t.query(250, 350), Some(1));
    assert_eq!(t.query(50, 350), Some(3));
    assert_eq!(t.query(50, 350_000), Some(3));

    // TODO: use step_by when it's stable
    for i in (1..10) {
        t.insert(i * 1000, 1000);
    }

    assert_eq!(t.query(1000, 3000), Some(3000));
    assert_eq!(t.query(1500, 5700), Some(4000));
    assert_eq!(t.query(5500, 9999), Some(4000));
    assert_eq!(t.query(50, 350_000), Some(9003));
    assert_eq!(t.query(250, 900_000), Some(9001));

    t.insert(900_200, 100);
    t.insert(900_300, -1000);

    assert_eq!(t.query(50, 900_250), Some(9103));
    assert_eq!(t.query(250, 999_000), Some(8101));
}
