use std::cell::UnsafeCell;

use crate::number::Number;

#[derive(Debug)]
pub struct ParRow<T>
{
    pub(crate) data: UnsafeCell<Vec<T>>,
}

impl<T: Number> ParRow<T>
{
    pub fn new(data: Vec<T>) -> Self
    {
        ParRow {
            data: UnsafeCell::new(data),
        }
    }

    pub fn new_empty() -> Self
    {
        ParRow {
            data: UnsafeCell::new(vec![]),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self
    {
        ParRow {
            data: UnsafeCell::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn new_with_default(n: usize) -> Self
    {
        ParRow {
            data: UnsafeCell::new(vec![T::default(); n]),
        }
    }

    pub fn get_mut(&self) -> &mut Vec<T>
    {
        unsafe { &mut *self.data.get() }
    }

    pub fn get(&self) -> &Vec<T>
    {
        unsafe { &*self.data.get() }
    }
}

unsafe impl<T> Send for ParRow<T> {}
unsafe impl<T> Sync for ParRow<T> {}
