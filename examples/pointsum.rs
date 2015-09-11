extern crate interval_tree;

use interval_tree::pointsum::{PointSumTree};

fn main() {
    // create a new tree, which will represent segment (2, 11) 
    let mut t = PointSumTree::new(2, 11);
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
}
