use std::cell::UnsafeCell;

use crate::{mat::Mat, row::ParRow};

#[test]
fn test_mat_new()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let mat = Mat::new(data.clone());

    let mat_data = unsafe { &*mat.data.get() };

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
    let data = unsafe { &*mat.data.get() };
    assert_eq!(data.len(), 0);
}

#[test]
fn test_mat_new_single_row()
{
    let mat: Mat<i32> = Mat::new(vec![vec![1, 2, 3]]);
    let data = unsafe { &*mat.data.get() };
    assert_eq!(data.len(), 1);
    assert_eq!(unsafe { &*data[0].data.get() }, &vec![1, 2, 3]);
}

#[test]
fn test_mat_new_multiple_rows()
{
    let mat: Mat<i32> = Mat::new(vec![vec![1, 2], vec![3, 4]]);
    let data = unsafe { &*mat.data.get() };
    assert_eq!(data.len(), 2);
    assert_eq!(unsafe { &*data[0].data.get() }, &vec![1, 2]);
    assert_eq!(unsafe { &*data[1].data.get() }, &vec![3, 4]);
}

#[test]
fn test_check_col_consistency_empty()
{
    let data: Vec<Vec<i32>> = vec![];
    Mat::check_col_consistency(&data); // should not panic
}

#[test]
fn test_check_col_consistency_consistent()
{
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Mat::check_col_consistency(&data); // should not panic
}

#[test]
#[should_panic(expected = "Inconsistent column length")]
fn test_check_col_consistency_inconsistent()
{
    let data = vec![vec![1, 2, 3], vec![4, 5], vec![7, 8, 9]];
    Mat::check_col_consistency(&data); // should panic
}
