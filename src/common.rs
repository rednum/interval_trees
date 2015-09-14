extern crate num;

use self::num::traits::{Num};
use self::num::traits::{One};

pub fn mid<N: Num>(start: N, end: N) -> N {
    let one1:N = One::one();
    let one2:N = One::one();
    let two = one1 + one2;
    (start + end) / two
}
