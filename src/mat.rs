use std::cell::UnsafeCell;

use crate::{number::Number, row::ParRow};

pub struct Mat<T>
{
    pub(crate) rows: UnsafeCell<Vec<ParRow<T>>>,
}

unsafe impl<T> Send for Mat<T> {}
unsafe impl<T> Sync for Mat<T> {}

impl<T: Number> Mat<T>
{
    pub(crate) fn check_col_consistency(rows: &Vec<Vec<T>>)
    {
        if rows.is_empty() {
            return;
        }

        let col_len = rows[0].len();
        for row in rows {
            if row.len() != col_len {
                panic!("Inconsistent column length");
            }
        }

        return;
    }

    pub(crate) fn get_row(&self, i: usize) -> &Vec<T>
    {
        let rows = self.get_rows();
        rows[i].get()
    }

    pub(crate) fn get_rows(&self) -> &Vec<ParRow<T>>
    {
        unsafe { &*self.rows.get() }
    }

    pub fn shape(&self) -> (usize, usize)
    {
        let rows = self.get_rows();

        if rows.is_empty() {
            return (0, 0);
        }

        (rows.len(), rows[0].get().len())
    }

    pub(crate) fn get_elements_per_thread(&self, n_threads: usize) -> Vec<usize>
    {
        let (n_rows, n_cols) = self.shape();
        let n_elements = n_rows * n_cols;
        let mut elements_per_thread: Vec<usize> = vec![n_elements / n_threads; n_threads];

        if n_elements < n_threads {
            for i in 0..n_elements {
                elements_per_thread[i] = 1;
            }
            return elements_per_thread;
        }

        let mut surplus_elements = n_elements % n_threads;
        for i in 0..n_threads {
            if surplus_elements == 0 {
                break;
            }
            elements_per_thread[i] += 1;
            surplus_elements -= 1;
        }

        elements_per_thread
    }

    pub fn new(data: Vec<Vec<T>>) -> Self
    {
        Self::check_col_consistency(&data);

        let mut rows = Vec::new();
        for row in data {
            rows.push(ParRow::new(row));
        }

        Mat {
            rows: UnsafeCell::new(rows),
        }
    }
}
