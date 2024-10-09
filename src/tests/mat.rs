use crate::mat::Mat;

#[test]
fn test_mat_new()
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
fn test_send_sync()
{
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Mat<i32>>();
    assert_sync::<Mat<i32>>();
}

#[test]
fn test_mat_new_empty()
{
    let mat: Mat<i32> = Mat::new(vec![]);
    let data = unsafe { &*mat.rows.get() };
    assert_eq!(data.len(), 0);
}

#[test]
fn test_mat_new_single_row()
{
    let mat: Mat<i32> = Mat::new(vec![vec![1, 2, 3]]);
    let data = unsafe { &*mat.rows.get() };
    assert_eq!(data.len(), 1);
    assert_eq!(unsafe { &*data[0].data.get() }, &vec![1, 2, 3]);
}

#[test]
fn test_mat_new_multiple_rows()
{
    let mat: Mat<i32> = Mat::new(vec![vec![1, 2], vec![3, 4]]);
    let data = unsafe { &*mat.rows.get() };
    assert_eq!(data.len(), 2);
    assert_eq!(unsafe { &*data[0].data.get() }, &vec![1, 2]);
    assert_eq!(unsafe { &*data[1].data.get() }, &vec![3, 4]);
}

#[test]
fn test_check_col_consistency_empty()
{
    let data: Vec<Vec<i32>> = vec![];
    Mat::check_col_consistency(&data);
}

#[test]
fn test_check_col_consistency_consistent()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Mat::check_col_consistency(&data);
}

#[test]
#[should_panic(expected = "Inconsistent column length")]
fn test_check_col_consistency_inconsistent()
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
fn test_get_row()
{
    let mat = create_matrix();
    let row = mat.get_row(1);
    let expected_row = vec![4, 5, 6];
    assert_eq!(row, &expected_row);
}

#[test]
fn test_get_rows()
{
    let mat = create_matrix();
    let rows = mat.get_rows();
    let expected_rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    for i in 0..rows.len() {
        assert_eq!(rows[i].get(), &expected_rows[i]);
    }
}

#[test]
fn test_shape()
{
    let mat = create_matrix();
    let shape = mat.shape();
    let expected_shape = (3, 3);
    assert_eq!(shape, expected_shape);
}

#[test]
fn test_empty_mat_shape()
{
    let mat: Mat<i32> = Mat::new(vec![]);
    let shape = mat.shape();
    let expected_shape = (0, 0);
    assert_eq!(shape, expected_shape);
}

fn create_test_matrix(data: Vec<Vec<i32>>) -> Mat<i32>
{
    Mat::new(data)
}

#[test]
fn test_get_elements_per_thread()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mat = create_test_matrix(data);

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
fn test_get_elements_per_thread_empty_matrix()
{
    let data: Vec<Vec<i32>> = vec![];
    let mat = create_test_matrix(data);

    // Test with any number of threads for an empty matrix
    let elements_per_thread = mat.get_elements_per_thread(3);
    assert_eq!(elements_per_thread, vec![0, 0, 0]);
}

#[test]
fn test_get_elements_per_thread_one_element_matrix()
{
    let data = vec![vec![1]];
    let mat = create_test_matrix(data);

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
