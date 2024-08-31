use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

mod project;

pub use project::Project;

pub struct Write<P> {
    pointer: P,
}

impl<P> Write<P> {
    #[inline]
    pub unsafe fn new_unchecked(pointer: P) -> Write<P> {
        Write { pointer }
    }

    #[inline]
    pub fn into_inner(write: Write<P>) -> P {
        write.pointer
    }
}

impl<T: ?Sized> Write<&T> {
    #[inline]
    pub fn from_mut(r: &mut T) -> Write<&T> {
        Write { pointer: r }
    }
}

impl<P: Deref> Write<P> {
    #[inline]
    pub fn as_ref(&mut self) -> Write<&P::Target> {
        let target = self.pointer.deref();
        unsafe { Write::new_unchecked(target) }
    }
}

impl<'a, T: ?Sized> Write<&'a T>
where
    T: Project<'a>,
{
    #[inline]
    pub fn project(self) -> T::Target {
        T::project(self)
    }
}

impl<P, T: ?Sized> Write<P>
where
    P: Deref<Target = WriteCell<T>>,
{
    #[inline]
    pub fn write(&mut self) -> &mut T {
        unsafe { &mut *self.pointer.deref().value.get() }
    }
}

impl<P: Deref> Deref for Write<P> {
    type Target = P::Target;

    #[inline]
    fn deref(&self) -> &P::Target {
        self.pointer.deref()
    }
}

impl<P: DerefMut> DerefMut for Write<P> {
    #[inline]
    fn deref_mut(&mut self) -> &mut P::Target {
        self.pointer.deref_mut()
    }
}

#[repr(transparent)]
pub struct WriteCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for WriteCell<T> {}
unsafe impl<T: ?Sized + Send> Sync for WriteCell<T> {}

impl<T> WriteCell<T> {
    #[inline]
    pub fn new(value: T) -> WriteCell<T> {
        WriteCell {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let mut cell = WriteCell::new(3);

        let mut write = Write::from_mut(&mut cell);
        *write.write() = 4;

        let mut write_ref = write.as_ref();
        *write_ref.write() = 5;
    }

    #[test]
    fn project_write() {
        let mut cell = WriteCell::new(3);

        let mut write_inner = Write::from_mut(&mut cell);
        let write_outer = Write::from_mut(&mut write_inner);

        *write_outer.project().write() = 4;
    }
}
