//! # Number Types
//! A `NumberType` is any type which can be used for the countdown problem. This
//! requires Addition, Subtraction, Multiplication, Division.

use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign,
    },
};
/// A helper trait which defines the required methods for a number.
pub trait NumberType:
    Sized
    + Copy
    + Debug
    + Display
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + CheckedOperations
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Sum
    + Eq
    + 'static
    + Ord
{
    /// Number representing one
    const ONE: Self;
    /// Number representing zero (must be the addition identity)
    const ZERO: Self;
    /// Checks whether a number is prime
    fn is_prime(self) -> bool {
        let mut a = Self::ONE + Self::ONE;
        while (a * a) <= self {
            if a % self == Self::ZERO {
                return false;
            }
            a += Self::ONE;
        }
        true
    }
}
struct CountdownRange<T: NumberType> {
    start: T,
    end: T,
    inclusive: bool,
}
impl<T: NumberType> Iterator for CountdownRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end || (self.inclusive && (self.start == self.end))
        {
            let result = Some(self.start);
            self.start += T::ONE;

            result
        } else {
            None
        }
    }
}
/// A trait which represents a number system. The four basic operations are
/// defined.
pub trait NumberSystem<T: NumberType>: Clone + Copy + Debug {
    fn add(&self, one: T, other: T) -> Option<T>;
    fn sub(&self, one: T, other: T) -> Option<T>;
    fn mul(&self, one: T, other: T) -> Option<T>;
    fn div(&self, one: T, other: T) -> Option<T>;
}
/// A number system which represents normal arithmetic
#[derive(Clone, Copy, Debug)]
pub struct NormalNumberSystem;
impl<T: NumberType> NumberSystem<T> for NormalNumberSystem {
    fn add(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ZERO && other != T::ZERO)
            .then(|| one.checked_add(other))
            .flatten()
    }

    fn sub(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ZERO && other != T::ZERO)
            .then(|| one.checked_sub(other))
            .flatten()
    }

    fn mul(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ONE && other != T::ONE)
            .then(|| one.checked_mul(other))
            .flatten()
    }

    fn div(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ONE && other != T::ONE)
            .then(|| one.checked_div(other))
            .flatten()
    }
}
/// A number system which implements modular arithmetic
#[derive(Clone, Copy, Debug)]
pub struct ModularNumberSystem<T: NumberType>(T, bool);

impl<T: NumberType> ModularNumberSystem<T> {
    /// Creating a modular number system from a value.
    pub fn new(base: T) -> Self {
        Self(base, base.is_prime())
    }
    fn in_range(&self, t: &mut T) {
        while *t >= self.0 {
            *t -= self.0;
        }
        while *t < T::ZERO {
            *t += self.0;
        }
        debug_assert!(*t >= T::ZERO && *t < self.0)
    }
    fn t_into_range(&self, mut t: T) -> T {
        self.in_range(&mut t);
        t
    }
    fn pow(&self, t: T, n: T) -> T {
        let mut v = T::ONE;
        for _ in (CountdownRange {
            start: T::ZERO,
            end: n,
            inclusive: false,
        }) {
            v = self.t_into_range(v * t);
        }
        v
    }
    fn multiplicative_inverse(&self, t: T) -> T {
        self.pow(t, self.0 - T::ONE)
    }
}

impl<T: NumberType> NumberSystem<T> for ModularNumberSystem<T> {
    fn add(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        Some(self.t_into_range(one.checked_add(other)?))
    }

    fn sub(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        if one != other {
            self.add(one, self.0.checked_sub(other)?)
        } else {
            None
        }
    }

    fn mul(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        Some(self.t_into_range(one.checked_mul(other)?))
    }

    fn div(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        if !self.1 {
            return None;
        }
        self.mul(one, self.multiplicative_inverse(other))
    }
}
/// Checked operations to check for overflow.
pub trait CheckedOperations: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
}
macro_rules! to_function {
    ($f:ident) => {
        fn $f(self, rhs: Self) -> Option<Self> {
            Self::$f(self, rhs)
        }
    };
}
macro_rules! impl_checked_operations {
    ($t:ident) => {
        impl CheckedOperations for $t {
            to_function!(checked_add);
            to_function!(checked_sub);
            to_function!(checked_mul);
            to_function!(checked_div);
        }
    };
}
macro_rules! impl_countdown_number_type {
    ($t:ident) => {
        impl NumberType for $t {
            const ONE: Self = 1;
            const ZERO: Self = 0;
        }
    };
}
impl_checked_operations!(u8);
impl_checked_operations!(u16);
impl_checked_operations!(u32);
impl_checked_operations!(u64);
impl_checked_operations!(u128);
impl_checked_operations!(usize);
impl_checked_operations!(i8);
impl_checked_operations!(i16);
impl_checked_operations!(i32);
impl_checked_operations!(i64);
impl_checked_operations!(i128);
impl_checked_operations!(isize);

impl_countdown_number_type!(u8);
impl_countdown_number_type!(u16);
impl_countdown_number_type!(u32);
impl_countdown_number_type!(u64);
impl_countdown_number_type!(u128);
impl_countdown_number_type!(usize);
impl_countdown_number_type!(i8);
impl_countdown_number_type!(i16);
impl_countdown_number_type!(i32);
impl_countdown_number_type!(i64);
impl_countdown_number_type!(i128);
impl_countdown_number_type!(isize);
