extern crate interval_tree;

use interval_tree::segmentpoint::{SegmentPointTree};

fn main() {
    // In this example, I will show how to use the segment-point 
    // tree to implement a data structure for storing intervals
    // and answering queries about them. Each interval has a 
    // beginning, end (ints) and weight (also, intervals have no 
    // "holes" - they cover all numbers greater or equal to beginning
    // and smaller or equal to end). The beginning and end must be
    // within bounds supported upon the construction of the tree
    // Each query has one argument: point P. The answer to the 
    // query should be the sum of all intervals containg point P.
    // 
    // A naive approach would be to represent the interval as
    // an array, then upon inserting segment [p, q] with weight w,
    // the insert would be something like:
    //
    // for i in (p..q):
    //     interval[i] += w;
    // 
    // Querying would be simple returning value of array in given
    // point.
    // 
    // The problem with this solution is that each insert is
    // O(interval length), which may not be great (though insert
    // is O(1)). Using the tree below, both insert and query are
    // O(log max interval length)
    //
    //
    // Those are bounds within which all inserts and queries
    // must be contained. It shouldn't be a problem to have
    // a tree of bounds [-10^9, 10^9] - the nodes are constructed
    // lazily upon insertion of nodes.
    //

    let bounds = (2, 11);

    // Initial value of every point inside bounds
    let initial_value = 0;

    // Function to "combine" interval in each point - just
    // sum them. 
    let f = Box::new(|x: &i64, y: &i64| x + y);

    // We will use a segment-point tree - the one where
    // we put segments, and ask about points.
    let mut t = SegmentPointTree::new(bounds.0, bounds.1, initial_value, f);

    // the segment looks now like this:
    // positions: | 2| 3| 4| 5| 6| 7| 8| 9|10|11|  
    //   weights: | 0| 0| 0| 0| 0| 0| 0| 0| 0| 0|

    // store segment (1, 5) of weight 1:
    t.insert(3, 7, 1);

    // the segment looks now like this:
    // positions: | 2| 3| 4| 5| 6| 7| 8| 9|10|11|  
    //   weights: | 0| 1| 1| 1| 1| 1| 0| 0| 0| 0|

    assert_eq!(t.query(2), Some(0));
    assert_eq!(t.query(4), Some(1));
    assert_eq!(t.query(5), Some(1));
    assert_eq!(t.query(9), Some(0));

    // (15, 21) is not inside (2, 11), so this would panic:
    // t.insert(15, 21, 1);
    // (-2, 1) is als not inside (2, 11), so this would panic, too:
    // t.insert(-2, 1, 1);


    // store more segments:

    t.insert(2, 4, 1);
    // positions: | 2| 3| 4| 5| 6| 7| 8| 9|10|11|  
    //   weights: | 1| 2| 2| 1| 1| 1| 0| 0| 0| 0|

    assert_eq!(t.query(2), Some(1));
    assert_eq!(t.query(4), Some(2));
    assert_eq!(t.query(5), Some(1));
    assert_eq!(t.query(9), Some(0));

    t.insert(7, 9, 5);
    // positions: | 2| 3| 4| 5| 6| 7| 8| 9|10|11|  
    //   weights: | 1| 2| 2| 1| 1| 6| 5| 5| 0| 0|

    assert_eq!(t.query(2), Some(1));
    assert_eq!(t.query(4), Some(2));
    assert_eq!(t.query(5), Some(1));
    assert_eq!(t.query(7), Some(6));
    assert_eq!(t.query(9), Some(5));
   
    // querying outside range will yield None
    assert_eq!(t.query(-10), None);
    assert_eq!(t.query(10000), None);

    println!("All asserts OK.");
}
