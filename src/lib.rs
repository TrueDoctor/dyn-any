#![doc(html_root_url = "http://docs.rs/const-default/1.0.0")]
#![cfg_attr(feature = "unstable-docs", feature(doc_cfg))]

#[cfg(feature = "derive")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "derive")))]
pub use dyn_any_derive::DynAny;

use std::any::TypeId;

pub trait DynAny<'a> {
    fn type_id(&self) -> TypeId;
}

impl<'a> DynAny<'a> for f32 {
    fn type_id(&self) -> TypeId {
        TypeId::of::<f32>()
    }
}
impl<'a, T: 'static> DynAny<'a> for &T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<&'static T>()
    }
}
pub fn downcast_ref<'a, V: StaticType<'a>>(
    i: &'a dyn DynAny<'a>,
) -> Option<&'a V> {
    if i.type_id() == std::any::TypeId::of::<<V as StaticType>::Static>() {
        // SAFETY: caller guarantees that T is the correct type
        let ptr = i as *const dyn DynAny<'a> as *const V;
        Some(unsafe { &*ptr })
    } else {
        None
    }
}

pub trait StaticType<'a>: 'a {
    type Static: 'static + ?Sized;
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }
}

pub trait StaticTypeSized<'a>: 'a {
    type Static: 'static;
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }
}
impl<'a, T: StaticTypeSized<'a>> StaticType<'a> for T {
    type Static = <T as StaticTypeSized<'a>>::Static;
}
pub trait StaticTypeClone<'a>: 'a {
    type Static: 'static + Clone;
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }
}
impl<'a, T: StaticTypeClone<'a>> StaticTypeSized<'a> for T {
    type Static = <T as StaticTypeClone<'a>>::Static;
}

macro_rules! impl_type {
    ($($id:ident$(<$($(($l:lifetime, $s:lifetime)),*|)?$($T:ident),*>)?),*) => {
        $(
        impl<'a, $($($T: 'a + $crate::StaticTypeSized<'a> + Sized,)*)?> $crate::StaticTypeSized<'a> for $id $(<$($($l,)*)?$($T, )*>)?{
            type Static = $id$(<$($($s,)*)?$(<$T as $crate::StaticTypeSized<'a>>::Static,)*>)?;
        }
        )*
    };
}
impl<'a, T: Clone + StaticTypeClone<'a>> StaticTypeClone<'a>
    for std::borrow::Cow<'a, T>
{
    type Static = std::borrow::Cow<'static, <T as StaticTypeSized<'a>>::Static>;
}
impl<'a, T: StaticTypeSized<'a>> StaticTypeSized<'a> for *const [T] {
    type Static = *const [<T as StaticTypeSized<'a>>::Static];
}
impl<'a, T: StaticTypeSized<'a>> StaticTypeSized<'a> for *mut [T] {
    type Static = *mut [<T as StaticTypeSized<'a>>::Static];
}
impl<'a, T: StaticTypeSized<'a>> StaticTypeSized<'a> for &'a [T] {
    type Static = &'static [<T as StaticTypeSized<'a>>::Static];
}
impl<'a> StaticTypeSized<'a> for &'a str {
    type Static = &'static str;
}
impl<'a> StaticTypeSized<'a> for () {
    type Static = ();
}
impl<'a, T: 'a + StaticTypeClone<'a>> StaticTypeClone<'a> for &'a T {
    type Static = &'static <T as StaticTypeClone<'a>>::Static;
}
impl<'a, T: StaticTypeSized<'a>, const N: usize> StaticTypeSized<'a>
    for [T; N]
{
    type Static = [<T as StaticTypeSized<'a>>::Static; N];
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
           BinaryHeap<T>, ManuallyDrop<T>, PhantomData<T>, PhantomPinned,Empty<T>,
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
        impl<'dyn_any, $($t: StaticTypeSized<'dyn_any>,)*> StaticTypeSized<'dyn_any> for ($($t,)*) {
            type Static = ($(<$t as $crate::StaticTypeSized<'dyn_any>>::Static,)*);
        }
    };
    ($($t:ident)*) => {
        impl_tuple! { @rec _t $($t)* }
    };
}

impl_tuple! {
    A B C D E F G H I J K L
}
