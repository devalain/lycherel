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
fn lycherel<T: Num + std::fmt::Octal>(num: T, base: T) {
    let mut l = num;
    loop {
        //println!("{:o}", l);
        if is_palindrome(&l, &base) {
            break;
        }
        l += mirror(&l.clone(), &base);
    }
}

// Lychrel numbers found
// base-2 : 10110
// base-8 : 1775
// base-10: 196
// base-16: 19D
fn main() {
    let mut i = Integer::from(1);
    let base = Integer::from(8);
    loop {
        println!("\n\n{:o}\n", i);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        lycherel(i.clone(), base.clone());
        i += 1;
    }
}
