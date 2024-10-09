use std::{cell::UnsafeCell, sync::Arc, thread};

use crate::{number::Number, row::ParRow};

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
            let unsafe_row = row_ptr.get_mut();
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
                let unsafe_row = row.get_mut();
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
fn new_empty()
{
    let row: ParRow<f32> = ParRow::new_empty();
    assert_eq!(*unsafe { &*row.data.get() }, vec![]);
}

#[test]
fn new_with_capacity()
{
    let capacity = 10;
    let row: ParRow<f32> = ParRow::new_with_capacity(capacity);
    let vec_ref = unsafe { &*row.data.get() };
    assert_eq!(vec_ref.capacity(), capacity);
    assert!(vec_ref.is_empty());
}

#[test]
fn new_with_default()
{
    let n = 5;
    let row: ParRow<f32> = ParRow::new_with_default(n);
    assert_eq!(*unsafe { &*row.data.get() }, vec![f32::default(); n]);
}

#[test]
fn get()
{
    let mock_data = ParRow::new(vec![1.0, 2.0, 3.0, 4.0]);
    let result = mock_data.get();
    assert_eq!(result, &vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn get_len()
{
    let row = ParRow {
        data: UnsafeCell::new(vec![1, 2, 3, 4, 5]),
    };
    assert_eq!(row.get_len(), 5);

    let empty_row = ParRow::<i32> {
        data: UnsafeCell::new(vec![]),
    };
    assert_eq!(empty_row.get_len(), 0);

    let single_element_row = ParRow {
        data: UnsafeCell::new(vec![42]),
    };
    assert_eq!(single_element_row.get_len(), 1);
}
