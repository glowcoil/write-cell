use std::ops::Deref;

use super::Write;

pub trait Project<'a> {
    type Target;

    fn project(this: Write<&'a Self>) -> Self::Target;
}

impl<'a, P: Deref> Project<'a> for Write<P>
where
    P::Target: 'a,
{
    type Target = Write<&'a P::Target>;

    #[inline]
    fn project(this: Write<&'a Self>) -> Self::Target {
        let target = this.pointer.pointer.deref();
        unsafe { Write::new_unchecked(target) }
    }
}
