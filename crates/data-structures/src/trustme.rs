#![allow(clippy::missing_transmute_annotations)]

/// Changes the lifetime of the given reference.
pub unsafe fn decouple_lt<'a, T: ?Sized>(x: &T) -> &'a T {
    std::mem::transmute(x)
}

/// Changes the lifetime of the given mutable reference.
pub unsafe fn decouple_lt_mut<'a, T: ?Sized>(x: &mut T) -> &'a mut T {
    std::mem::transmute(x)
}
