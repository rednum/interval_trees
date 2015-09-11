# Introduction
This is a variant of interval tree data structure. This name may not be technically correct - please see last paragraph for explanation.

This tree allows following operations (each runs in O(log N) complexity):
- insert a segment with some weight
- count sum of weights of segments in a given point

Therefore the name pointsum tree - it allows you to find sum of intervals in a point.

# Usage

Short example below. Please refer to examples/pointsum.rs for bigger example.

```rust
use interval_tree::pointsum::{PointSumTree};

fn main() {
    let mut t = PointSumTree::new(1, 10);
    assert_eq!(t.bounds(), (1, 10));
    t.insert(1, 5, 1);
    t.insert(5, 6, 21);
    assert_eq!(t.query(1), Some(1));
    assert_eq!(t.query(2), Some(1));
    assert_eq!(t.query(5), Some(21));
    assert_eq!(t.query(6), Some(20));
    assert_eq!(t.query(9), Some(0));
}
```

# Naming

I'm not sure what is the correct name for this data structure - I learned it
in my algorithm class under polish name "drzewo przedziałowe", which could be
translated to something like "interval tree". However, it seems like it
slightly differs from standard interval tree implementation - instead
of storing all intervals, I keep only interval weight in nodes (which
makes it less flexible but improves complexity). It's also rather easy to 
generalize this structure to work like canonized interval tree.
