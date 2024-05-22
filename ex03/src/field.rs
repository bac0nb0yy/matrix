use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Field:
    Add<Output = Self>
    + AddAssign
    + Add
    + Sub<Output = Self>
    + SubAssign
    + Sub
    + Mul<Output = Self>
    + MulAssign
    + Mul
    + Div<Output = Self>
    + DivAssign
    + Div
    + Neg<Output = Self>
    + Into<f64>
    + Copy
    + Zero
    + One
{
}

impl<
        T: Add<Output = Self>
            + AddAssign
            + Add
            + Sub<Output = Self>
            + SubAssign
            + Sub
            + Mul<Output = Self>
            + MulAssign
            + Mul
            + Div<Output = Self>
            + DivAssign
            + Div
            + Neg<Output = Self>
            + Into<f64>
            + Copy
            + Zero
            + One,
    > Field for T
{
}
