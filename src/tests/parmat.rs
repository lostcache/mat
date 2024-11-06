use crate::parmat::ParMat;

#[test]
fn test_dot_product()
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
fn test_dot_product_dimension_mismatch()
{
    let mat1_data = vec![vec![(1), (2)], vec![(3), (4)]];
    let mat2_data = vec![vec![(2), (0)]];

    let mat1 = ParMat::from(mat1_data);
    let mat2 = ParMat::from(mat2_data);

    ParMat::dot(&mat1, &mat2);
}
