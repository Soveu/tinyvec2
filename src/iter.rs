use crate::ArrayVec;

use core::slice;
use core::mem;

pub struct IntoIter<T: Default, const N: usize> {
    data: ArrayVec<[T; N]>,
    front_offset: u16,
}

impl<T: Default, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front_offset == self.data.len {
            return None;
        }
        let val = self.data.get_mut(self.front_offset as usize)?;
        self.front_offset += 1;
        Some(mem::take(val))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.data.len - self.front_offset;
        let hint = hint as usize;
        (hint, Some(hint))
    }
}

impl<T: Default, const N: usize> IntoIterator for ArrayVec<[T; N]> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: self,
            front_offset: 0,
        }
    }
}

impl<'a, T: Default> IntoIterator for &'a ArrayVec<[T]> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T: Default> IntoIterator for &'a mut ArrayVec<[T]> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
