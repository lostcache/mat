use std::cell::UnsafeCell;

use crate::{number::Number, row::ParRow};

pub struct Mat<T>
{
    pub(crate) data: UnsafeCell<Vec<ParRow<T>>>,
}

unsafe impl<T> Send for Mat<T> {}
unsafe impl<T> Sync for Mat<T> {}

impl<T: Number> Mat<T>
{
    pub(crate) fn check_col_consistency(data: &Vec<Vec<T>>)
    {
        if data.is_empty() {
            return;
        }

        let col_len = data[0].len();
        for row in data {
            if row.len() != col_len {
                panic!("Inconsistent column length");
            }
        }

        return;
    }

    pub fn new(data: Vec<Vec<T>>) -> Self
    {
        Self::check_col_consistency(&data);

        let mut rows = Vec::new();
        for row in data {
            rows.push(ParRow::new(row));
        }
        Mat {
            data: UnsafeCell::new(rows),
        }
    }
}
