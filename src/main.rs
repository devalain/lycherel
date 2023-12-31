use std::{
    cmp::{PartialEq, PartialOrd},
    fmt::{Debug, Display},
    ops::{AddAssign, DivAssign, MulAssign, Rem},
};

macro_rules! assemble_trait {
    {trait $name:ident { $($trait:path),+ $(,)? }} => {
        trait $name: $($trait+)+ {}
        impl<T> $name for T where
            T: $($trait+)+ {}
    }
}
assemble_trait! {
    trait Num {
        Clone,
        Rem<Output = Self>,
        AddAssign,
        MulAssign,
        DivAssign,
        PartialOrd,
        Display,
        Debug,
        Sized
    }
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
    let mut i = 1_u128;
    let base = 8;
    loop {
        println!("{:o}", i);
        lycherel(i, base);
        i += 1;
    }
}
