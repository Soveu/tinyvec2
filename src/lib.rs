#![forbid(unsafe_code)]
#![no_std]

mod iter;
pub use iter::*;

use core::mem;
use core::num::NonZeroUsize;
use core::fmt;
use core::ops;

pub trait Array: AsMut<[<Self as Array>::Item]> {
    type Item: Default;
}

impl<T: Default, const N: usize> Array for [T; N] {
    type Item = T;
}

impl<T: Default> Array for [T] {
    type Item = T;
}

pub struct ArrayVec<A: Array + ?Sized> {
    pub len: u16,
    pub data: A,
}

impl<T: Default, const N: usize> ops::Deref for ArrayVec<[T; N]> {
    type Target = ArrayVec<[T]>;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl<T: Default, const N: usize> ops::DerefMut for ArrayVec<[T; N]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl<T: Default> ops::Deref for ArrayVec<[T]> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[..self.len()]
    }
}

impl<T: Default> ops::DerefMut for ArrayVec<[T]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..self.len as usize]
    }
}

impl<T: Default> ArrayVec<[T]> {
    pub fn capacity(&self) -> usize {
        self.data.len()
    }
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.truncate(0)
    }
    pub fn truncate(&mut self, new_len: usize) {
        let items = match self.get_mut(..new_len) {
            Some(x) => x,
            None => return,
        };

        items.iter_mut()
            .for_each(|x| *x = Default::default());
        self.len = new_len as u16;
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = match self.len.checked_sub(1) {
            Some(x) => x,
            None => return None,
        };

        self.len = index;
        let item = &mut self.data.as_mut()[index as usize];
        return Some(mem::take(item));
    }
    pub fn try_push(&mut self, item: T) -> Result<(), ExtendError<T>> {
        let len = self.len();
        if let Some(new) = self.data.as_mut().get_mut(len) {
            *new = item;
            return Ok(());
        }

        return Err(ExtendError {
            missing_capacity: NonZeroUsize::new(1).unwrap(),
            item,
        });
    }
    pub fn push(&mut self, item: T) {
        self.try_push(item).unwrap()
    }
}

pub struct ExtendError<T> {
    pub missing_capacity: NonZeroUsize,
    pub item: T,
}

impl<T> fmt::Debug for ExtendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtendError")
            .field("missing_capacity", &self.missing_capacity)
            .finish()
    }
}
