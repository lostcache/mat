use std::cell::UnsafeCell;

pub trait Float {}

impl Float for f32 {}
impl Float for f64 {}

#[derive(Debug)]
pub struct ParRow<T>
{
    data: UnsafeCell<Vec<T>>,
}

impl<T: Float> ParRow<T>
{
    pub fn get_mut(&self) -> &mut Vec<T>
    {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl<T> Send for ParRow<T> {}
unsafe impl<T> Sync for ParRow<T> {}

#[cfg(test)]
mod row_tests
{
    use std::{sync::Arc, thread};

    use super::*;

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
}
