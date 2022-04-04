#![doc(html_root_url = "http://docs.rs/const-default/1.0.0")]
#![cfg_attr(feature = "unstable-docs", feature(doc_cfg))]

#[cfg(feature = "derive")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "derive")))]
pub use dyn_any_derive::DynAny;

pub trait DynAny<'a>: 'a {
    type Static: 'static + ?Sized;
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }
}
pub trait DynAnySized<'a>: 'a {
    type Static: 'static;
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }
}
impl<'a, T: DynAnySized<'a>> DynAny<'a> for T {
    type Static = <T as DynAnySized<'a>>::Static;
}
pub trait DynAnyClone<'a>: 'a {
    type Static: 'static + Clone;
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }
}
impl<'a, T: DynAnyClone<'a>> DynAnySized<'a> for T {
    type Static = <T as DynAnyClone<'a>>::Static;
}

macro_rules! impl_type {
    ($($id:ident$(<$($(($l:lifetime, $s:lifetime)),*|)?$($T:ident),*>)?),*) => {
        $(
        impl<'a, $($($T: 'a + $crate::DynAnySized<'a> + Sized,)*)?> $crate::DynAny<'a> for $id $(<$($($l,)*)?$($T, )*>)?{
            type Static = $id$(<$($($s,)*)?$(<$T as $crate::DynAnySized<'a>>::Static,)*>)?;
        }
        )*
    };
}
impl<'a, T: Clone + DynAnyClone<'a>> DynAnyClone<'a>
    for std::borrow::Cow<'a, T>
{
    type Static = std::borrow::Cow<'static, <T as DynAnySized<'a>>::Static>;
}
impl<'a, T: DynAnySized<'a>> DynAnySized<'a> for *const [T] {
    type Static = *const [<T as DynAnySized<'a>>::Static];
}
impl<'a, T: DynAnySized<'a>> DynAnySized<'a> for *mut [T] {
    type Static = *mut [<T as DynAnySized<'a>>::Static];
}
impl<'a, T: DynAnySized<'a>> DynAnySized<'a> for &'a [T] {
    type Static = &'static [<T as DynAnySized<'a>>::Static];
}
impl<'a> DynAnySized<'a> for &'a str {
    type Static = &'static str;
}
impl<'a> DynAnySized<'a> for () {
    type Static = ();
}
impl<'a, T: DynAnySized<'a>, const N: usize> DynAnySized<'a> for [T; N] {
    type Static = [<T as DynAnySized<'a>>::Static; N];
}

use core::{
    cell::{Cell, RefCell, UnsafeCell},
    iter::Empty,
    marker::{PhantomData, PhantomPinned},
    mem::{ManuallyDrop, MaybeUninit},
    num::Wrapping,
    time::Duration,
};
use std::{
    collections::*,
    sync::{atomic::*, *},
    vec::Vec,
};

impl_type!(Option<T>,Result<T, E>,Cell<T>,UnsafeCell<T>,RefCell<T>,MaybeUninit<T>,
           Vec<T>, String, BTreeMap<K,V>,BTreeSet<V>, LinkedList<T>, VecDeque<T>,
           BinaryHeap<T>, ManuallyDrop<T>, PhantomData<T>, PhantomPinned<>,Empty<T>,
           Wrapping<T>, Duration, Once, Mutex<T>, RwLock<T>,  bool, f32, f64, char,
           u8, AtomicU8, u16,AtomicU16, u32,AtomicU32, u64,AtomicU64, usize,AtomicUsize,
           i8,AtomicI8, i16,AtomicI16, i32,AtomicI32, i64,AtomicI64, isize,AtomicIsize,
            i128, u128, AtomicBool, AtomicPtr<T>
);
macro_rules! impl_tuple {
    (@rec $t:ident) => { };
    (@rec $_:ident $($t:ident)+) => {
        impl_tuple! { @impl $($t)* }
        impl_tuple! { @rec $($t)* }
    };
    (@impl $($t:ident)*) => {
        impl<'dyn_any, $($t: DynAnySized<'dyn_any>,)*> DynAnySized<'dyn_any> for ($($t,)*) {
            type Static = ($(<$t as $crate::DynAnySized<'dyn_any>>::Static,)*);
        }
    };
    ($($t:ident)*) => {
        impl_tuple! { @rec _t $($t)* }
    };
}

impl_tuple! {
    A B C D E F G H I J K L
}
