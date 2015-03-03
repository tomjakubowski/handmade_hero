use std::iter::Iterator;
use std::marker;

pub struct CircularIterMut<'a, T: 'a> {
    buff: *mut T,
    len: usize,
    index: usize,
    ind0: usize,
    _marker: marker::PhantomData<&'a mut T>,
}

impl<'a, T> CircularIterMut<'a, T> {
    pub fn new(buff: &'a mut Box<[T]>, index: usize) -> CircularIterMut<'a, T> {
        let len = buff.len();
        CircularIterMut {
            buff: buff.as_mut_ptr(),
            len: len,
            index: index % len,
            ind0: 0,
            _marker: marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for CircularIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.ind0 < self.len {
            unsafe {
                let ptr = self.buff.offset((self.index % self.len) as isize);
                self.index += 1;
                self.ind0 += 1;
                ptr.as_mut()
            }
        } else {
            None
        }
    }
}
