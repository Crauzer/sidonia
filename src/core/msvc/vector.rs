use std::{mem, ops, slice};

#[repr(C)]
#[derive(Debug)]
pub struct StdVector<T> {
    start: *mut T,
    end: *mut T,
    capacity: *mut T,
}

pub struct StdVectorIterator<T> {
    vector: StdVector<T>,
    index: usize,
}

impl<T> StdVector<T> {
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl<T> ops::Index<usize> for StdVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.len() {
            let item_size = mem::size_of::<T>();
            unsafe { self.start.offset(item_size as isize).as_ref().unwrap() }
        } else {
            panic!("index was outside the bounds of the vector");
        }
    }
}
impl<T> ops::IndexMut<usize> for StdVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.len() {
            let item_size = mem::size_of::<T>();
            unsafe { self.start.offset(item_size as isize).as_mut().unwrap() }
        } else {
            panic!("index was outside the bounds of the vector");
        }
    }
}

impl<T> ops::Deref for StdVector<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.start, self.len()) }
    }
}
impl<T> ops::DerefMut for StdVector<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.start, self.len()) }
    }
}

impl<'a, T> IntoIterator for &'a StdVector<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut StdVector<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
