use crate::number::Number;

pub(crate) struct ParMat<T: Number>
{
    pub(crate) rows: std::cell::UnsafeCell<Vec<Vec<T>>>,
}

unsafe impl<T: Number> Sync for ParMat<T> {}
unsafe impl<T: Number> Send for ParMat<T> {}

impl<T: Number> ParMat<T>
{
    pub(crate) fn validate_vec(vec: &Vec<Vec<T>>)
    {
        assert!(vec.len() != 0, "Cannot init empty mat");
        let cols = vec[0].len();
        assert!(cols != 0, "Cannot init empty mat");
        for row in vec {
            assert!(row.len() == cols, "Inconsistant col size");
        }
    }

    pub fn from(vec: Vec<Vec<T>>) -> Self
    {
        Self::validate_vec(&vec);
        Self {
            rows: std::cell::UnsafeCell::new(vec),
        }
    }

    pub(crate) fn get(&self) -> &Vec<Vec<T>>
    {
        unsafe { &*self.rows.get() }
    }

    pub(crate) fn get_mut(&mut self) -> &mut Vec<Vec<T>>
    {
        unsafe { &mut *self.rows.get() }
    }

    pub fn shape(&self) -> (usize, usize)
    {
        let rows = self.rows.get();
        unsafe { ((&(*rows)).len(), (&(*rows))[0].len()) }
    }

    pub(crate) fn get_elements_per_thread(&mut self, n_threads: usize) -> Vec<usize>
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

    pub(crate) fn get_batch_indices(
        &mut self,
        n_threads: usize,
    ) -> Vec<((usize, usize), (usize, usize))>
    {
        let (_, n_cols) = self.shape();
        let elements_per_thread = self.get_elements_per_thread(n_threads);
        let batch_linear_indices = self.get_batch_linear_indices(elements_per_thread);
        let mut batch_indices: Vec<((usize, usize), (usize, usize))> =
            vec![((0, 0), (0, 0)); n_threads];

        for i in 0..batch_linear_indices.len() {
            let (liner_start_index, linear_end_index) = batch_linear_indices[i];
            let ((start_i, start_j), (end_i, end_j)): ((usize, usize), (usize, usize)) = (
                (liner_start_index / n_cols, liner_start_index % n_cols),
                (linear_end_index / n_cols, linear_end_index % n_cols),
            );
            batch_indices[i] = ((start_i, start_j), (end_i, end_j));
        }
        batch_indices
    }

    pub(crate) fn loc(&self, i: usize, j: usize) -> T
    {
        let (n_rows, n_cols) = self.shape();
        assert!(i < n_rows, "Index out of range");
        assert!(j < n_cols, "Index out of range");
        self.get()[i][j]
    }

    pub(crate) fn set(&mut self, i: usize, j: usize, val: T)
    {
        let (n_rows, n_cols) = self.shape();
        assert!(i < n_rows, "Index out of range");
        assert!(j < n_cols, "Index out of range");
        self.get_mut()[i][j] = val;
    }

    pub(crate) fn dot_batch(
        &mut self,
        mat1: &ParMat<T>,
        mat2: &ParMat<T>,
        start_i: usize,
        start_j: usize,
        end_i: usize,
        end_j: usize,
    )
    {
        let (n_rows, n_cols) = mat1.shape();
        for i in start_i..=end_i {
            for j in start_j..=end_j {
                let mut sum = T::default();
                for k in 0..n_cols {
                    sum += mat1.loc(i, k) * mat2.loc(k, j);
                }
                self.set(i, j, sum);
            }
        }
    }

    pub fn dot(mat1: &ParMat<T>, mat2: &ParMat<T>) -> ParMat<T>
    {
        let (mat1_row, mat1_col) = mat1.shape();
        let (mat2_row, mat2_col) = mat2.shape();
        let (res_row, res_col) = (mat1_row, mat2_col);

        assert!(
            mat1_col == mat2_row,
            "Matrix dimensions do not match for dot product"
        );

        let mut result: ParMat<T> = ParMat::from(vec![vec![T::default(); res_col]; res_row]);

        let n_threads = 8;
        let batch_indices = result.get_batch_indices(n_threads);
        let mut prev_batch_index = ((usize::MAX, usize::MAX), (usize::MAX, usize::MAX));

        let mut handles = Vec::new();
        let result_ptr = &mut result as *mut ParMat<T>;
        let mat1_ptr = mat1 as *const ParMat<T>;
        let mat2_ptr = mat2 as *const ParMat<T>;

        for i in 0..n_threads {
            if prev_batch_index == batch_indices[i] {
                break;
            }

            let ((start_i, start_j), (end_i, end_j)) = batch_indices[i];
            unsafe {
                let mat1_ref: &ParMat<T> = &*mat1_ptr;
                let mat2_ref: &ParMat<T> = &*mat2_ptr;
                let res_ref: &mut ParMat<T> = &mut *result_ptr;
                handles.push(std::thread::spawn(move || {
                    res_ref.dot_batch(mat1_ref, mat2_ref, start_i, start_j, end_i, end_j);
                }));
            }
            prev_batch_index = batch_indices[i];
        }

        for handle in handles {
            handle.join().unwrap();
        }

        result
    }
}
