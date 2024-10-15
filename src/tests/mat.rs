use crate::mat::Mat;

#[test]
fn mat_new()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let mat = Mat::new(data.clone());

    let mat_data = unsafe { &*mat.rows.get() };

    assert_eq!(mat_data.len(), data.len());
    for (row, par_row) in data.iter().zip(mat_data) {
        assert_eq!(unsafe { &*par_row.data.get() }, row);
    }
}

#[test]
fn send_sync()
{
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Mat<i32>>();
    assert_sync::<Mat<i32>>();
}

#[test]
#[should_panic(expected = "Row in matrix is empty")]
fn mat_new_empty()
{
    let mat: Mat<i32> = Mat::new(vec![]);
    let data = unsafe { &*mat.rows.get() };
    assert_eq!(data.len(), 0);
}

#[test]
fn mat_new_single_row()
{
    let mat: Mat<i32> = Mat::new(vec![vec![1, 2, 3]]);
    let data = unsafe { &*mat.rows.get() };
    assert_eq!(data.len(), 1);
    assert_eq!(unsafe { &*data[0].data.get() }, &vec![1, 2, 3]);
}

#[test]
fn mat_new_multiple_rows()
{
    let mat: Mat<i32> = Mat::new(vec![vec![1, 2], vec![3, 4]]);
    let data = unsafe { &*mat.rows.get() };
    assert_eq!(data.len(), 2);
    assert_eq!(unsafe { &*data[0].data.get() }, &vec![1, 2]);
    assert_eq!(unsafe { &*data[1].data.get() }, &vec![3, 4]);
}

#[test]
#[should_panic(expected = "Row in matrix is empty")]
fn check_col_consistency_empty()
{
    let data: Vec<Vec<i32>> = vec![];
    Mat::check_col_consistency(&data);
}

#[test]
fn check_col_consistency_consistent()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Mat::check_col_consistency(&data);
}

#[test]
#[should_panic(expected = "Inconsistent column length")]
fn check_col_consistency_inconsistent()
{
    let data = vec![vec![1, 2, 3], vec![4, 5], vec![7, 8, 9]];
    Mat::check_col_consistency(&data);
}

fn create_matrix() -> Mat<i32>
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Mat::new(data)
}

#[test]
fn get_row()
{
    let mat = create_matrix();
    let row = mat.get_row(1);
    let expected_row = vec![4, 5, 6];
    assert_eq!(row, &expected_row);
}

#[test]
fn get_rows()
{
    let mat = create_matrix();
    let rows = mat.get_rows();
    let expected_rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    for i in 0..rows.len() {
        assert_eq!(rows[i].get(), &expected_rows[i]);
    }
}

#[test]
fn shape()
{
    let mat = create_matrix();
    let shape = mat.shape();
    let expected_shape = (3, 3);
    assert_eq!(shape, expected_shape);
}

fn create_matrix_from_data(data: Vec<Vec<i32>>) -> Mat<i32>
{
    Mat::new(data)
}

#[test]
fn get_elements_per_thread()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mat = create_matrix_from_data(data);

    // Test with 1 thread
    let elements_per_thread = mat.get_elements_per_thread(1);
    assert_eq!(elements_per_thread, vec![9]);

    // Test with 3 threads
    let elements_per_thread = mat.get_elements_per_thread(3);
    assert_eq!(elements_per_thread, vec![3, 3, 3]);

    // Test with 4 threads (more threads than elements)
    let elements_per_thread = mat.get_elements_per_thread(4);
    assert_eq!(elements_per_thread, vec![3, 2, 2, 2]);

    // Test with 2 threads
    let elements_per_thread = mat.get_elements_per_thread(2);
    assert_eq!(elements_per_thread, vec![5, 4]);

    // Test with 5 threads (more threads than elements)
    let elements_per_thread = mat.get_elements_per_thread(5);
    assert_eq!(elements_per_thread, vec![2, 2, 2, 2, 1]);

    // Test with 6 threads (more threads than elements)
    let elements_per_thread = mat.get_elements_per_thread(6);
    assert_eq!(elements_per_thread, vec![2, 2, 2, 1, 1, 1]);

    // Test with 9 threads (equal to number of elements)
    let elements_per_thread = mat.get_elements_per_thread(9);
    assert_eq!(elements_per_thread, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);

    // Test with 10 threads (more threads than elements)
    let elements_per_thread = mat.get_elements_per_thread(10);
    assert_eq!(elements_per_thread, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0]);
}

#[test]
fn get_elements_per_thread_one_element_matrix()
{
    let data = vec![vec![1]];
    let mat = create_matrix_from_data(data);

    // Test with 1 thread
    let elements_per_thread = mat.get_elements_per_thread(1);
    assert_eq!(elements_per_thread, vec![1]);

    // Test with 2 threads (more threads than elements)
    let elements_per_thread = mat.get_elements_per_thread(2);
    assert_eq!(elements_per_thread, vec![1, 0]);

    // Test with 3 threads (more threads than elements)
    let elements_per_thread = mat.get_elements_per_thread(3);
    assert_eq!(elements_per_thread, vec![1, 0, 0]);
}

#[test]
fn get_batch_linear_indices_single_thread()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mat = create_matrix_from_data(data);
    let elements_per_thread = vec![6];
    let result = mat.get_batch_linear_indices(elements_per_thread);
    assert_eq!(result, vec![(0, 5)]);
}

#[test]
fn get_batch_linear_indices_even_distribution()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mat = create_matrix_from_data(data);
    let elements_per_thread = vec![3, 3];
    let result = mat.get_batch_linear_indices(elements_per_thread);
    assert_eq!(result, vec![(0, 2), (3, 5)]);
}

#[test]
fn get_batch_linear_indices_uneven_distribution()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mat = create_matrix_from_data(data);
    let elements_per_thread = vec![4, 4, 1];
    let result = mat.get_batch_linear_indices(elements_per_thread);
    assert_eq!(result, vec![(0, 3), (4, 7), (8, 8)]);
}

#[test]
fn get_batch_linear_indices_more_threads_than_elements()
{
    let data = vec![vec![1, 2, 3]];
    let mat = create_matrix_from_data(data);
    let elements_per_thread = vec![1, 1, 1, 0];
    let result = mat.get_batch_linear_indices(elements_per_thread);
    assert_eq!(result, vec![(0, 0), (1, 1), (2, 2), (2, 2)]);
}

#[test]
fn test_get_batch_indices_single_thread()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mat = Mat::new(data);
    let indices = mat.get_batch_indices(1);
    assert_eq!(indices, vec![((0, 0), (2, 2))]);
}

#[test]
fn test_get_batch_indices_multiple_threads()
{
    let data = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    let mat = Mat::new(data);
    let indices = mat.get_batch_indices(2);
    assert_eq!(indices, vec![((0, 0), (1, 1)), ((2, 0), (3, 1))]);
}

#[test]
fn test_get_batch_indices_more_threads_than_elements()
{
    let data = vec![vec![1, 2], vec![3, 4]];
    let mat = Mat::new(data);
    let indices = mat.get_batch_indices(5);
    assert_eq!(
        indices,
        vec![
            ((0, 0), (0, 0)),
            ((0, 1), (0, 1)),
            ((1, 0), (1, 0)),
            ((1, 1), (1, 1)),
            ((1, 1), (1, 1))
        ]
    );
}
