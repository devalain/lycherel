use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{Debug, Display, UpperHex};
use std::ops::{AddAssign, DivAssign, MulAssign, Rem};

use rug::Integer;

trait Num:
    Clone
        + Rem<Output = Self> 
        + AddAssign 
        + MulAssign 
        + DivAssign 
        + PartialOrd 
        + Display 
        + Debug 
        + Sized
{
}
impl<T> Num for T where
    T: Clone
        + Rem<Output = Self>
        + AddAssign
        + MulAssign
        + DivAssign
        + PartialOrd
        + Display
        + Debug
        + Sized
{
}

fn mirror<T: Num>(num: &T, base: &T) -> T {
    let zero = base.clone() % base.clone();
    let mut r = zero.clone();
    let mut num = num.clone();
    while num > zero {
        r *= base.clone();
        r += num.clone() % base.clone();
        num /= base.clone();
    }
    r
}
fn is_palindrome<T: Num + PartialEq>(num: &T, base: &T) -> bool {
    num == &mirror(num, base)
}
fn lycherel<T: Num + UpperHex>(num: T, base: T) {
    let mut l = num;
    loop {
        // println!("{:X}", l);
        if is_palindrome(&l, &base) {
            break;
        }
        l += mirror(&l.clone(), &base);
    }
}

// Lychrel numbers found
// base-10: 196
// base-16: 19D
// base-2 : 10110
fn main() {
    let mut i = Integer::from(1);
    let base = Integer::from(2);
    loop {
        println!("{:b}", i);
        lycherel(i.clone(), base.clone());
        i += 1;
    }
}
