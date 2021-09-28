use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{Debug, Display};
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
fn lycherel<T: Num>(num: T, base: T) {
    let mut l = num;
    loop {
        println!("{}", l);
        if is_palindrome(&l, &base) {
            break;
        }
        l += mirror(&l.clone(), &base);
    }
}

fn main() {
    lycherel(Integer::from(196), Integer::from(10));
}
