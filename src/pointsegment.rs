extern crate num;

use self::num::traits::{Num};
use self::num::traits::{One};
use common::{mid};
use std::fmt::{Debug};
use std::io::stdout;
use std::io::Write;

pub struct PointSegmentTree<N, P>{
    root: Node<N, P>,
    lower_bound: N, 
    upper_bound: N, 
    default: P,
    combine: Box<F<P>>
}

#[derive(PartialEq, Eq, Debug)]
struct Node<N, P> {
    start: N,
    end: N,
    value: P, 
    left: Option<Box<Node<N, P>>>,
    right: Option<Box<Node<N, P>>>,
}

pub type F<P> = Fn(&P, &P) -> P;

impl<N: Debug+Num+Clone+Ord, P: Debug+Clone> PointSegmentTree<N, P> {
    pub fn new(lower_bound: N, upper_bound: N, default_value: P,
               combine: Box<F<P>>) -> Self 
    {
        if lower_bound > upper_bound {
            panic!("Invalid bounds (lower_bound must not be greater than upper_bound)");
        }
        let node = Node::new(lower_bound.clone(),
                             upper_bound.clone(),
                             &default_value);
            
        PointSegmentTree {
            default:  default_value,
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            combine: combine,
            root: node
        }
    }

    pub fn insert(&mut self, point_n: N, point_data: P) {
        if point_n < self.lower_bound || point_n > self.upper_bound {
           panic!("Attempted insert out of tree bounds"); 
        }
        self.root.insert(point_n, point_data, &self.default, 
                         &*self.combine);
    }

    pub fn query(&self, start_q: N, end_q: N) -> Option<P> {
        if end_q < start_q || start_q < self.lower_bound || end_q > self.upper_bound {
            None
        } else {
            Some(self.root.query(start_q, end_q, &*self.combine, self.default.clone()))
        }
    }

    pub fn bounds(&self) -> (N, N) {
        (self.lower_bound.clone(), self.upper_bound.clone())
    }

}


impl<N: Num+Clone+Ord+Debug, P: Debug+Clone> Node<N, P> {
    fn new(start: N, end: N, default_value: &P) -> Self { 
        Node {
            start: start,
            end: end,
            value: default_value.clone(),
            left: None,
            right: None,
        }
    }

    fn new_son(start: N, end: N, default_value: &P) -> Option<Box<Self>> {
        Some(Box::new(Node::new(start, end, default_value)))
    }

    fn query(&self, start_q: N, end_q: N, combine: &F<P>, acc: P) -> P {
        if self.start == start_q && self.end == end_q {
            stdout().flush();
            return combine(&self.value, &acc);
        }
        let mid = mid(self.start.clone(), self.end.clone());

        if end_q <= mid {
            match self.left {
                None => acc,
                Some(ref n) => n.query(start_q, end_q, combine, acc)
            }
        } else if start_q > mid {
            match self.right {
                None => acc,
                Some(ref n) => n.query(start_q, end_q, combine, acc)
            }
        } else {
            // split
            let acc_l = match self.left {
                None => acc,
                Some(ref n) => n.query(start_q, mid.clone(), combine, acc)
            };
            let acc_r = match self.right {
                None => acc_l,
                Some(ref n) => n.query(mid + One::one(), end_q, combine, acc_l)
            };
            acc_r
        }
    }

    fn insert(&mut self, point_n: N, point_data: P,
              default: &P, combine: &F<P>) {
        let mid = mid(self.start.clone(), self.end.clone());
        self.value = combine(&self.value, &point_data);
        if self.start == self.end {
            // leaf
            return;
        }
        if point_n <= mid {
            if self.left.is_none() {
                self.left = Node::new_son(self.start.clone(), mid, default);
            }
            self.left.as_mut().map(|n| n.insert(point_n, point_data, default, combine));
        } else {
            if self.right.is_none() {
                self.right = Node::new_son(mid + One::one(), self.end.clone(), default);
            }
            self.right.as_mut().map(|n| n.insert(point_n, point_data, default, combine));
        }
    }
}
