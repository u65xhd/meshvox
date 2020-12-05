use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, PartialEq)]
pub(crate) struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + One + Copy> Vector3<T> {
    #[inline]
    pub(crate) fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub(crate) fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline]
    pub(crate) fn cross(&self, rhs: &Self) -> Self {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Self::new(x, y, z)
    }
    #[inline]
    pub(crate) fn x_axis() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }
    #[inline]
    pub(crate) fn y_axis() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }
    #[inline]
    pub(crate) fn z_axis() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
}

impl<T: Copy> Clone for Vector3<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
