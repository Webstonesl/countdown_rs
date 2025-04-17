use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign},
};

pub trait CountdownNumberType:
    Sized
    + Copy
    + Debug
    + Display
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Sum
    + Eq
    + Ord
{
    const ONE: Self;
    const ZERO: Self;
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
impl CountdownNumberType for usize {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl CountdownNumberType for u8 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl CountdownNumberType for u16 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl CountdownNumberType for u32 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl CountdownNumberType for u64 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
pub trait CountdownNumberBaseType: CountdownNumberType + 'static {}
impl<T: CountdownNumberType + 'static> CountdownNumberBaseType for T {}
struct CountdownRange<T: CountdownNumberBaseType> {
    start: T,
    end: T,
    inclusive: bool,
}
impl<T: CountdownNumberBaseType> Iterator for CountdownRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end || (self.inclusive && (self.start == self.end)) {
            let result = Some(self.start);
            self.start += T::ONE;

            result
        } else {
            None
        }
    }
}
pub trait NumberSystem<T: CountdownNumberBaseType>: Clone + Copy + Debug {
    fn add(&self, one: T, other: T) -> Option<T>;
    fn sub(&self, one: T, other: T) -> Option<T>;
    fn mul(&self, one: T, other: T) -> Option<T>;
    fn div(&self, one: T, other: T) -> Option<T>;
}
#[derive(Clone, Copy, Debug)]
pub struct NormalNumberSystem;
impl<T: CountdownNumberBaseType> NumberSystem<T> for NormalNumberSystem {
    fn add(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ZERO && other != T::ZERO).then(|| one + other)
    }

    fn sub(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ZERO && other != T::ZERO).then(|| one - other)
    }

    fn mul(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ONE && other != T::ONE).then(|| one * other)
    }

    fn div(&self, one: T, other: T) -> Option<T> {
        (one > other && one != T::ONE && other != T::ONE).then(|| one / other)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ModularNumberSystem<T: CountdownNumberBaseType>(T, bool);

impl<T: CountdownNumberBaseType> ModularNumberSystem<T> {
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
        assert!(*t >= T::ZERO && *t < self.0)
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

impl<T: CountdownNumberBaseType> NumberSystem<T> for ModularNumberSystem<T> {
    fn add(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        Some(self.t_into_range(one + other))
    }

    fn sub(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        if one != other {
            self.add(one, self.0 - other)
        } else {
            None
        }
    }

    fn mul(&self, one: T, other: T) -> Option<T> {
        debug_assert!(one < self.0 && one >= T::ZERO);
        debug_assert!(other < self.0 && other >= T::ZERO);
        Some(self.t_into_range(one * other))
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
