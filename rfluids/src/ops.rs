use std::ops::{Div, Mul};

pub(crate) fn mul<R, L, O, E>(lhs: Result<R, E>, rhs: Result<L, E>) -> Result<O, E>
where
    R: Mul<L, Output = O>,
{
    lhs.and_then(|l| rhs.map(|r| l * r))
}

pub(crate) fn div<R, L, O, E>(lhs: Result<R, E>, rhs: Result<L, E>) -> Result<O, E>
where
    R: Div<L, Output = O>,
{
    lhs.and_then(|l| rhs.map(|r| l / r))
}
