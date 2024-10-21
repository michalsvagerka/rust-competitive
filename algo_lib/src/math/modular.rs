use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Field<const M: u32>(pub u32);

pub type FieldMod = Field<1000000007>;
pub type FieldFft = Field<998244353>;

impl<const M: u32> Default for Field<M> {
    fn default() -> Self {
        Self(0)
    }
}

impl<const M: u32> Display for Field<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<const M: u32> Field<M> {
    /// Generates a vector of factorials modulo M from 0 to max, inclusive.
    pub fn fact(max: usize) -> Vec<Self> {
        let mut v = vec![Self(1)];
        v.reserve(max);
        let mut x = 1u64;
        for i in 1..=max {
            x *= i as u64;
            x *= M as u64;
            v.push(Self(x as u32))
        }

        v
    }

    /// Generates a vector of inverse factorials modulo M from 0 to max, inclusive.
    pub fn invfact(max: usize) -> Vec<Self> {
        let mut tmp = 1u64;
        let mut v = Vec::new();
        v.resize(max + 1, Self(1));
        for i in 2..=max {
            tmp *= i as u64;
            tmp %= M as u64;
        }

        tmp = Self::pow(tmp as u32, M - 2) as u64;
        for i in (2..=max).rev() {
            v[i] = Self(tmp as u32);
            tmp *= i as u64;
            tmp %= M as u64;
        }

        v
    }

    pub fn inv(self) -> Self {
        Self(Self::pow(self.0, M - 2))
    }

    fn pow(a: u32, mut p: u32) -> u32 {
        let mut r: u64 = 1;
        let mut e = a as u64;
        while p > 0 {
            if p & 1 == 1 {
                r = (r * e) % M as u64;
            }
            e = (e * e) % M as u64;
            p >>= 1;
        }
        r as u32
    }
}

impl<const M: u32> From<u64> for Field<M> {
    fn from(value: u64) -> Self {
        Field((value % M as u64) as u32)
    }
}
impl<const M: u32> From<i64> for Field<M> {
    fn from(value: i64) -> Self {
        Field((value % M as i64) as u32)
    }
}


impl<const M: u32> From<i32> for Field<M> {
    fn from(value: i32) -> Self {
        Field(value as u32)
    }
}
impl<const M: u32> From<u32> for Field<M> {
    fn from(value: u32) -> Self {
        Field(value)
    }
}

impl<const M: u32> Neg for Field<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            self
        } else {
            Self(M - self.0)
        }
    }
}
impl<const M: u32, T: Into<Field<M>>> AddAssign<T> for Field<M> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into().0;
        if self.0 >= M {
            self.0 -= M;
        }
    }
}


impl<const M: u32, T: Into<Field<M>>> SubAssign<T> for Field<M> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 += M;
        self.0 -= rhs.into().0;
        if self.0 >= M {
            self.0 -= M;
        }
    }
}

impl<const M: u32, T: Into<Field<M>>> MulAssign<T> for Field<M> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 = ((self.0 as u64 * rhs.into().0 as u64) % M as u64) as u32;
    }
}

impl<const M: u32, T: Into<Field<M>>> DivAssign<T> for Field<M> {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: T) {
        *self *= rhs.into().inv();
    }
}

impl<const M: u32, T: Into<Field<M>>> Add<T> for Field<M> {
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs.into();
        self
    }
}

impl<const M: u32, T: Into<Field<M>>> Sub<T> for Field<M> {
    type Output = Self;

    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs.into();
        self
    }
}

impl<const M: u32, T: Into<Field<M>>> Mul<T> for Field<M> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs.into();
        self
    }
}
impl<const M: u32> Div for Field<M> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const M: u32> Readable for Field<M> {
    fn read(input: &mut Input) -> Self {
        Self(input.read::<u32>())
    }
}

impl<const M: u32> Writable for Field<M> {
    fn write(&self, output: &mut Output) {
        output.print(self.0)
    }
}
