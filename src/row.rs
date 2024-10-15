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
        assert!(!data.is_empty(), "Row in a Matrix cannot be empty");
        ParRow {
            data: UnsafeCell::new(data),
        }
    }

    pub(crate) fn get_mut_ref(&self) -> &mut Vec<T>
    {
        unsafe { &mut *self.data.get() }
    }

    pub(crate) fn get_ref(&self) -> &Vec<T>
    {
        unsafe { &*self.data.get() }
    }

    pub(crate) fn loc(&self, i: usize) -> &T
    {
        assert!(i < self.len(), "Index out of bounds");
        &self.get_ref()[i]
    }

    pub(crate) fn len(&self) -> usize
    {
        self.get_ref().len()
    }
}

unsafe impl<T> Send for ParRow<T> {}
unsafe impl<T> Sync for ParRow<T> {}
