use self::Tree::{Node, Leaf};

#[derive(PartialEq, Eq, Debug)]
pub struct PointSumTree {
    root: Tree,
    lower_bound: i64,
    upper_bound: i64
}

#[derive(PartialEq, Eq, Debug)]
enum Tree {
    Node {start: i64, end: i64, value: i64, 
          left: Option<Box<Tree>>, right: Option<Box<Tree>>},
    Leaf(i64)
}

impl PointSumTree {
    pub fn new(lower_bound: i64, upper_bound: i64) -> Self {
        if lower_bound > upper_bound {
            panic!("Invalid bounds: {} {} (lower bound must not be greater than upper bound)", lower_bound, upper_bound);
        }
        PointSumTree {
            root: Tree::new(lower_bound, upper_bound),
            lower_bound: lower_bound,
            upper_bound: upper_bound
        }
    }

    pub fn insert(&mut self, start: i64, end: i64, value: i64) {
        if start < self.lower_bound || end > self.upper_bound {
            panic!("Invalid range: [{}, {}] (the tree bounds are [{}, {}]", 
                    start, end, self.lower_bound, self.upper_bound);
        }
        self.root.insert(start, end, value);
    }

    pub fn query(&self, point: i64) -> Option<i64> {
        if self.lower_bound > point || point > self.upper_bound {
            return None
        }
        Some(self.root.query(point, 0))
    }

    pub fn bounds(&self) -> (i64, i64) {
        (self.lower_bound, self.upper_bound)
    }
}

fn mid(start: i64, end: i64) -> i64 {
    start + (end - start) / 2 + 1
}

impl Tree {
    fn new(start: i64, end: i64) -> Self {
        if start == end {
            Leaf(0)
        } else {
            Node {start: start, end: end, left: None, right: None, value: 0}
        }
    }

    fn new_son(start: i64, end: i64) -> Option<Box<Tree>> {
        Some(Box::new(Tree::new(start, end)))
    }

    fn insert(&mut self, start_i: i64, end_i: i64, value_i: i64) {
        match *self {
            Tree::Node {start: ref start_n, end: ref end_n,
                 value: ref mut value_n, ref mut left, ref mut right} => {
                let mid_n = mid(*start_n, *end_n);
                if *start_n == start_i && *end_n == end_i {
                    *value_n += value_i; 
                    return;
                }
                if end_i < mid_n {
                    if left.is_none() {
                        *left = Tree::new_son(*start_n, (mid_n - 1)); 
                    }
                    left.as_mut()
                        .map(|n| n.insert(start_i, end_i, value_i));
                } else if start_i >= mid_n {
                    if right.is_none() {
                        *right = Tree::new_son(mid_n, *end_n);
                    }
                    right.as_mut()
                        .map(|n| n.insert(start_i, end_i, value_i));
                } else {
                    if left.is_none() {
                        *left = Tree::new_son(*start_n, (mid_n - 1));
                    }
                    left.as_mut()
                        .map(|n| n.insert(start_i, (mid_n - 1), value_i));

                    if right.is_none() {
                        *right = Tree::new_son(mid_n, *end_n);
                    }

                    right.as_mut()
                        .map(|n| n.insert(mid_n, end_i, value_i));
                }
            }
            Leaf(ref mut value) => {
                *value += value_i;
            }
        }
   }

   fn query(&self, point: i64, acc: i64) -> i64 {
        match *self {
            Leaf(value) => {
                acc + value
            }
            Node {ref start, ref end, ref value, ref left, 
                  ref right} => {
                let mid = mid(*start, *end);
                let acc = value + acc;
                if point < mid {
                    left.as_ref().map_or(acc, |n| n.query(point, acc))
                } else {
                    right.as_ref().map_or(acc, |n| n.query(point, acc))
                }
            }
        }
   }
}
