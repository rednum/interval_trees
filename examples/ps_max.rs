extern crate interval_tree;

use interval_tree::pointsegment::{PointSegmentTree};

use std::cmp::{max};

fn main() {
    // In this example, I will show how to use the point-segment
    // tree to implement a data structure for storing points and
    // answering queries about them. Each point belongs in 
    // predetermined bounds (supplied to tree when creating it).
    // It also has an integral value, bigger than zero. Each query 
    // has two arguments - beginning in end. Both beginning and 
    // end also must be within the tree's bounds. The answer to 
    // query about segment (p, q) is the value of maximum point
    // contained in segment [p, q] (ends included).
    // 
    // A naive approach would be to have a big array to represent 
    // all points. Then inserting a point of value x at position 
    // p would be something like:
    // array[p] += x
    // 
    // Then, query(p, q) could be support like:
    // let mut result = -INFINITY;
    // for i in (p..q):
    //     result = max(result, array[i]);
    // return result;
    // 
    // In this approach, insertion is O(1), however query is O(N).
    // (For N - bounds size). Using point-segment tree, we can
    // support both this operations in O(log(N)).

    let bounds = (-3, 5);

    // Initial value of every point inside bounds
    let initial_value = 0;

    // Function to "combine" points in each sub-segment - take max.
    let f = Box::new(|x: &i64, y: &i64| max(*x, *y));

    // We will use a point-segment tree - the one where
    // we put segments, and ask about points.
    let mut t = PointSegmentTree::new(bounds.0, bounds.1, initial_value, f);

    // The segment in bounds looks like this:
    // positions: -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4 | 5
    //    values:  0 |  0 |  0 | 0 | 0 | 0 | 0 | 0 | 0 

    t.insert(1, 3);

    // The segment in bounds looks like this:
    // positions: -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4 | 5
    //    values:  0 |  0 |  0 | 0 | 3 | 0 | 0 | 0 | 0 

    t.insert(2, 5);

    // The segment in bounds looks like this:
    // positions: -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4 | 5
    //    values:  0 |  0 |  0 | 0 | 3 | 5 | 0 | 0 | 0 

    t.insert(-1, 10);

    // The segment in bounds looks like this:
    // positions: -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4 | 5
    //    values:  0 |  0 | 10 | 0 | 3 | 5 | 0 | 0 | 0 

    // Max value in segment [1, 1] is in point 1 and equals 3
    assert_eq!(t.query(1, 1), Some(3));
    // Max value in segment [2, 2] is in point 2 and equals 5
    assert_eq!(t.query(2, 2), Some(5));
    // Max value in segment [-1, -1] is in point -1 and equals 10
    assert_eq!(t.query(-1, -1), Some(10));
    // Max value in segment [0, 0] is in point 0 and equals 0 (which is initial value
    assert_eq!(t.query(0, 0), Some(0));

    
    // The biggest value in [0, 2] is in point 2; it equals 5
    assert_eq!(t.query(0, 2), Some(5));
    
    // The biggest value in [0, 5] is in point 2; it equals 5
    assert_eq!(t.query(0, 5), Some(5));

    // The biggest value in [-1, 5] is in point -1; it equals 10
    assert_eq!(t.query(-1, 5), Some(10));

    // The biggest value in [-3, 5] is in point -1; it equals 10
    assert_eq!(t.query(-1, 5), Some(10));

    // Queries outside bounds will be None:
    assert_eq!(t.query(-10, 5), None);
    assert_eq!(t.query(0, 500), None);
    assert_eq!(t.query(-100, 500), None);

    // Insert outside bounds would panic:
    // t.insert(100, 10);
    // t.insert(-100, 10);

    // Some more inserts and queries
    t.insert(3, 4);
    t.insert(4, 7);
    t.insert(0, 7);
    // The segment in bounds looks like this:
    // positions: -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4 | 5
    //    values:  0 |  0 | 10 | 7 | 3 | 5 | 4 | 7 | 0 

    assert_eq!(t.query(0, 1), Some(7));
    assert_eq!(t.query(1, 2), Some(5));
    assert_eq!(t.query(0, 2), Some(7));
    assert_eq!(t.query(2, 4), Some(7));
    assert_eq!(t.query(3, 5), Some(7));
    assert_eq!(t.query(-1, 5), Some(10));

    t.insert(-1, 2);
    t.insert(0, 1);
    t.insert(-3, 20);

    // The segment in bounds looks like this:
    // positions: -3 | -2 | -1 | 0 | 1 | 2 | 3 | 4 | 5
    //    values: 20 |  0 |  2 | 1 | 3 | 5 | 4 | 7 | 0 

    assert_eq!(t.query(0, 1), Some(3));
    assert_eq!(t.query(1, 2), Some(5));
    assert_eq!(t.query(0, 2), Some(5));
    assert_eq!(t.query(2, 4), Some(7));
    assert_eq!(t.query(3, 5), Some(7));
    assert_eq!(t.query(-1, 5), Some(7));
    assert_eq!(t.query(-3, 4), Some(20));

    println!("All asserts OK.");
}
