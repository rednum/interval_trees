use self::Tree::{Node, Leaf};

#[derive(PartialEq, Eq, Debug)]
pub struct PointMaxTree {
    root: Tree
}

#[derive(PartialEq, Eq, Debug)]
enum Tree {
    Node {start: i64, end: i64, value: i64, 
          left: Option<Box<Tree>>, right: Option<Box<Tree>>},
    Leaf(i64)
}

impl PointMaxTree {
    pub fn new(lower_bound: i64, upper_bound: i64) -> Self {
        if lower_bound > upper_bound {
            panic!("Invalid bounds: {} {} (lower bound must not be greater than upper bound)", lower_bound, upper_bound);
        }
        PointMaxTree {
            root: Tree::new(&lower_bound, &upper_bound)
        }
    }

    pub fn insert(self: &mut PointMaxTree, start: i64, end: i64, value: i64) {
        if let Node { start: start_n, end: end_n, .. } = self.root {
            if start < start_n || end > end_n {
                panic!("Invalid range: [{}, {}] (the tree bounds are [{}, {}]", 
                start, end, start_n, end_n);
            }
        }
        self.root.insert(&start, &end, &value);
    }

    pub fn query(self: &PointMaxTree, point: i64) -> Option<i64> {
        if let Node { start, end, .. } = self.root {
            if start > point || point > end {
                return None
            }
        }
        Some(self.root.query(&point, &0))
    }
}

fn mid(start: i64, end: i64) -> i64 {
    start + (end - start) / 2 + 1
}

impl Tree {
    fn new(start: &i64, end: &i64) -> Self {
        if start == end {
            Leaf(0)
        } else {
            Node {start: *start, end: *end, left: None, right: None, value: 0}
        }
    }

    fn new_son(start: &i64, end: &i64) -> Option<Box<Tree>> {
        Some(Box::new(Tree::new(start, end)))
    }

    fn insert(self: &mut Tree, start_i: &i64, end_i: &i64, value_i: &i64) {
        match *self {
            Tree::Node {start: ref start_n, end: ref end_n,
                 value: ref mut value_n, ref mut left, ref mut right} => {
                println!("{} {} <- {} {}", start_n, end_n, start_i, end_i);
                let mid_n = mid(*start_n, *end_n);
                if *start_n == *start_i && *end_n == *end_i {
                    *value_n += *value_i; 
                    return;
                }
                if *end_i < mid_n {
                    if left.is_none() {
                        *left = Tree::new_son(start_n, &(mid_n - 1)); 
                    }
                    left.as_mut()
                        .map(|n| n.insert(start_i, end_i, value_i));
                } else if *start_i >= mid_n {
                    if right.is_none() {
                        *right = Tree::new_son(&mid_n, end_n);
                    }
                    right.as_mut()
                        .map(|n| n.insert(start_i, end_i, value_i));
                } else {
                    if left.is_none() {
                        *left = Tree::new_son(start_n, &(mid_n - 1));
                    }
                    left.as_mut()
                        .map(|n| n.insert(start_i, &(mid_n - 1), value_i));

                    if right.is_none() {
                        *right = Tree::new_son(&mid_n, end_n);
                    }

                    right.as_mut()
                        .map(|n| n.insert(&mid_n, end_i, value_i));
                }
            }
            Leaf(ref mut value) => {
                *value += *value_i;
            }
        }
   }

   fn query(self: &Tree, point: &i64, acc: &i64) -> i64 {
        match *self {
            Leaf(value) => {
                acc + value
            }
            Node {ref start, ref end, ref value, ref left, 
                  ref right} => {
                let mid = mid(*start, *end);
                let acc = value + acc;
                if *point < mid {
                    left.as_ref().map_or(acc, |n| n.query(point, &acc))
                } else {
                    right.as_ref().map_or(acc, |n| n.query(point, &acc))
                }
            }
        }
   }
}
