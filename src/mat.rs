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
            panic!("Row in matrix is empty");
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

    pub(crate) fn get_batch_linear_indices(
        &self,
        elements_per_thread: Vec<usize>,
    ) -> Vec<(usize, usize)>
    {
        let mut batch_linear_end_indices: Vec<(usize, usize)> =
            vec![(0, 0); elements_per_thread.len()];
        // -1 because of 0-based indexing
        let (mut start, mut end): (usize, usize) = (0, elements_per_thread[0] - 1);
        batch_linear_end_indices[0] = (start, end);
        for i in 1..elements_per_thread.len() {
            if elements_per_thread[i] > 0 {
                start = end + 1;
                end = start + elements_per_thread[i] - 1;
            }
            batch_linear_end_indices[i] = (start, end)
        }
        batch_linear_end_indices
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
