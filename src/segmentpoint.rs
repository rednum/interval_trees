extern crate num;

use self::num::traits::{Num};
use self::num::traits::{One};
use common::{mid};

pub struct SegmentPointTree<N, S>{
    root: Node<N, S>,
    lower_bound: N, 
    upper_bound: N, 
    default: S,
    combine: Box<F<S>>
}

#[derive(PartialEq, Eq, Debug)]
struct Node<N, S> {
    start: N,
    end: N,
    value: S, 
    left: Option<Box<Node<N, S>>>,
    right: Option<Box<Node<N, S>>>,
}

pub type F<S> = Fn(&S, &S) -> S;

impl<N: Num+Clone+Ord, S: Clone> SegmentPointTree<N, S> {
    pub fn new(lower_bound: N, upper_bound: N, default_value: S,
               combine: Box<F<S>>) -> Self 
    {
        if upper_bound < lower_bound {
            panic!("Invalid bounds (lower_bound must not be greater than upper_bound)");
        }
        SegmentPointTree {
            lower_bound: lower_bound.clone(),
            upper_bound: upper_bound.clone(),
            default: default_value.clone(),
            root: Node::new(lower_bound, upper_bound, &default_value),
            combine: combine
        }
    }

    pub fn query(&self, point: N) -> Option<S> {
        let s = self.default.clone();
        if point > self.upper_bound || point < self.lower_bound {
            None
        } else {
            Some(self.root.query(point, &*self.combine, s))
        }
    }

    pub fn bounds(&self) -> (N, N) {
        return (self.lower_bound.clone(), self.upper_bound.clone())
    }

    pub fn insert(&mut self, start: N, end: N, segment: S) -> () {
        if start < self.lower_bound || end > self.upper_bound {
            panic!("Can't insert outside of bounds");
        }
        self.root.insert(start, end, &segment, &self.default, &*self.combine);
    }
}

impl<N: Num+Clone+Ord, S: Clone> Node<N, S> {
    fn new(start: N, end: N, default_value: &S) -> Self {
        Node {
            value: default_value.clone(),
            start: start,
            end: end,
            left: None,
            right: None,
        }
    }

    fn new_son(start: N, end: N, default_value: &S) -> Option<Box<Self>> {
        Some(Box::new(Node::new(start, end, &default_value)))
    }

    fn query(&self, point: N, combine: &F<S>, acc: S) -> S { 
        let acc2 = combine(&acc, &self.value);
        let mid_n = mid(self.start.clone(), self.end.clone());
        let ref son = if point <= mid_n {
            &self.left
        } else {
            &self.right
        };
        match son.as_ref() {
            None => acc2,   
            Some(n) => n.query(point, combine, acc2)
        }// .map_or(acc2, |n| n.query(point, combine, acc2))
    }

    fn insert(&mut self, start_s: N, end_s: N, value_s: &S, 
              default: &S, combine: &F<S>) -> () {
        if start_s == self.start && end_s == self.end {
            // ugh.
            self.value = combine(&self.value, value_s);
            return;
        }
        let mid_n = mid(self.start.clone(), self.end.clone());
        if end_s <= mid_n {
            // only left
            if self.left.is_none() {
                self.left = Node::new_son(self.start.clone(), mid_n, default);
            }
            self.left.as_mut().map(|n| n.insert(start_s, end_s, 
                                                value_s, default, combine));
        } else if start_s > mid_n {
            // only right
            if self.right.is_none() {
                self.right = Node::new_son(mid_n + One::one(), self.end.clone(), default);
            }
            self.right.as_mut().map(|n| n.insert(start_s, end_s, 
                                                 value_s, default, combine));
        } else {
            // both
            if self.left.is_none() {
                self.left = Node::new_son(self.start.clone(), mid_n.clone(), default);
            }
            self.left.as_mut().map(|n| n.insert(start_s, mid_n.clone(), 
                                                value_s, default, combine));
            if self.right.is_none() {
                self.right = Node::new_son(mid_n.clone() + One::one(), self.end.clone(), default);
            }
            self.right.as_mut().map(|n| n.insert(mid_n + One::one(), end_s, 
                                                 value_s, default, combine));
        }
    }
}

