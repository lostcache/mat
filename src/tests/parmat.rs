use crate::parmat::ParMat;

#[test]
fn dot_product()
{
    let mat1_data = vec![vec![(1), (2)], vec![(3), (4)]];
    let mat2_data = vec![vec![(2), (0)], vec![(1), (2)]];

    let mat1 = ParMat::from(mat1_data);
    let mat2 = ParMat::from(mat2_data);

    let expected_data = vec![vec![(4), (4)], vec![(10), (8)]];
    let expected = ParMat::from(expected_data);

    let result = ParMat::dot(&mat1, &mat2);

    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(result.loc(i, j), expected.loc(i, j));
        }
    }

    let mat1_data = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mat2_data = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let result = ParMat::dot(&ParMat::from(mat1_data), &ParMat::from(mat2_data));

    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(result.loc(i, j), 8);
        }
    }
}

#[test]
#[should_panic(expected = "Matrix dimensions do not match for dot product")]
fn dot_product_dimension_mismatch()
{
    let mat1_data = vec![vec![(1), (2)], vec![(3), (4)]];
    let mat2_data = vec![vec![(2), (0)]];

    let mat1 = ParMat::from(mat1_data);
    let mat2 = ParMat::from(mat2_data);

    ParMat::dot(&mat1, &mat2);
}

#[test]
fn validate_vec_non_empty()
{
    let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    ParMat::validate_vec(&vec);
}

#[test]
#[should_panic(expected = "Cannot init empty mat")]
fn validate_vec_empty_outer()
{
    let vec: Vec<Vec<i32>> = Vec::new();
    ParMat::validate_vec(&vec);
}

#[test]
#[should_panic(expected = "Cannot init empty mat")]
fn validate_vec_empty_inner()
{
    let vec: Vec<Vec<i32>> = vec![Vec::new()];
    ParMat::validate_vec(&vec);
}

#[test]
#[should_panic(expected = "Inconsistant col size")]
fn validate_vec_inconsistent_cols()
{
    let vec = vec![vec![1, 2], vec![3, 4, 5]];
    ParMat::validate_vec(&vec);
}

#[test]
fn test_matrix_from()
{
    let vec = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut matrix = ParMat::from(vec.clone());

    assert_eq!(*matrix.rows.get_mut(), vec);
}

#[test]
#[should_panic(expected = "Cannot init empty mat")]
fn test_matrix_from_empty()
{
    let vec: Vec<Vec<i32>> = Vec::new();
    let mut matrix = ParMat::from(vec.clone());

    assert_eq!(*matrix.rows.get_mut(), vec);
}

#[test]
#[should_panic(expected = "Cannot init empty mat")]
fn matrix_from_empty_col()
{
    let vec: Vec<Vec<i32>> = vec![vec![]];
    let mut matrix = ParMat::from(vec.clone());

    assert_eq!(*matrix.rows.get_mut(), vec);
}

#[test]
#[should_panic(expected = "Inconsistant col size")]
fn test_matrix_from_invalid()
{
    let vec = vec![vec![1, 2], vec![3]];
    ParMat::from(vec); // Assuming validate_vec will panic on invalid input
}
