use std::cell::UnsafeCell;

use crate::{number::Number, row::ParRow};

pub struct Mat<T>
{
    pub (crate) data: UnsafeCell<Vec<ParRow<T>>>,
}

unsafe impl<T> Send for Mat<T> {}
unsafe impl<T> Sync for Mat<T> {}

impl<T: Number> Mat<T>
{
    pub fn new(data: Vec<Vec<T>>) -> Self
    {
        let mut rows = Vec::new();
        for row in data {
            rows.push(ParRow::new(row));
        }
        Mat {
            data: UnsafeCell::new(rows),
        }
    }
}
