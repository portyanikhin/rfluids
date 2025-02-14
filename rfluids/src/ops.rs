use std::ops::{Div, Mul};

pub(crate) fn mul<L, R, O, E>(lhs: Result<L, E>, rhs: Result<R, E>) -> Result<O, E>
where
    L: Mul<R, Output = O>,
{
    lhs.and_then(|l| rhs.map(|r| l * r))
}

pub(crate) fn div<L, R, O, E>(lhs: Result<L, E>, rhs: Result<R, E>) -> Result<O, E>
where
    L: Div<R, Output = O>,
{
    lhs.and_then(|l| rhs.map(|r| l / r))
}
