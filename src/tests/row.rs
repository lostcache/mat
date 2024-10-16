use std::{cell::UnsafeCell, sync::Arc, thread};

use crate::row::ParRow;

#[test]
fn row_init()
{
    let row = ParRow {
        data: UnsafeCell::new(vec![1.0, 2.0, 3.0]),
    };
}

#[test]
fn row_sharing_across_threads()
{
    let row = ParRow {
        data: UnsafeCell::new(vec![]),
    };

    let row_ptr = Arc::new(row);

    let handles = (0..8).map(|i| {
        let row_ptr = row_ptr.clone();
        thread::spawn(move || {
            if i % 2 == 0 {
                thread::sleep(std::time::Duration::from_millis(50));
            }
            let unsafe_row = row_ptr.get_mut_ref();
            unsafe_row.push(i as f64);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn row_sharing_in_scoped_threads()
{
    let row = ParRow {
        data: UnsafeCell::new(vec![]),
    };

    for i in 0..8 {
        thread::scope(|s| {
            s.spawn(|| {
                if i % 2 == 0 {
                    thread::sleep(std::time::Duration::from_millis(50));
                }
                let unsafe_row = row.get_mut_ref();
                unsafe_row.push(0 as f64);
            });
        });
    }
}

#[test]
fn new()
{
    let data: Vec<f32> = vec![1.0, 2.0, 3.0];
    let row = ParRow::new(data.clone());
    assert_eq!(*unsafe { &*row.data.get() }, data);
}

#[test]
#[should_panic(expected = "Row in a Matrix cannot be empty")]
fn new_empty()
{
    let data: Vec<f32> = vec![];
    let row = ParRow::new(data);
}

#[test]
fn get_ref()
{
    let mock_data = ParRow::new(vec![1.0, 2.0, 3.0, 4.0]);
    let result = unsafe { &*mock_data.data.get() };
    assert_eq!(result, &vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn len()
{
    let row = ParRow {
        data: UnsafeCell::new(vec![1, 2, 3, 4, 5]),
    };
    assert_eq!(row.len(), 5);

    let empty_row = ParRow::<i32> {
        data: UnsafeCell::new(vec![]),
    };
    assert_eq!(empty_row.len(), 0);

    let single_element_row = ParRow {
        data: UnsafeCell::new(vec![42]),
    };
    assert_eq!(single_element_row.len(), 1);
}

#[test]
fn loc_valid_index()
{
    let data = vec![1, 2, 3];
    let row = ParRow::new(data);
    assert_eq!(row.loc(1), &2);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn loc_out_of_bounds()
{
    let data = vec![1, 2, 3];
    let row = ParRow::new(data);
    row.loc(3);
}
