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
/*
impl<'a, U: 'static> dyn DynAny<'a, Static = U> {
    pub fn downcast_ref<V: DynAny<'a>>(&self) -> Option<&'a V> {
        if self.type_id() == std::any::TypeId::of::<<V as DynAny>::Static>() {
            let ptr = self as *const Self as *const V;
            Some(unsafe { &*ptr })
        } else {
            None
        }
    }
    pub fn downcast_box<V: DynAny<'a>>(&self) -> Option<Box<V>> {
        if self.type_id() == std::any::TypeId::of::<<V as DynAny>::Static>() {
            let ptr = self as *const Self as *mut V;
            Some(unsafe { Box::from_raw(ptr) })
        } else {
            None
        }
    }
}
impl<'a, U: 'static> dyn DynAnySized<'a, Static = U> {
    pub fn downcast_ref<V: DynAnySized<'a>>(&self) -> Option<&'a V> {
        if self.type_id()
            == std::any::TypeId::of::<<V as DynAnySized>::Static>()
        {
            let ptr = self as *const Self as *const V;
            Some(unsafe { &*ptr })
        } else {
            None
        }
    }
    pub fn downcast_box<V: DynAnySized<'a>>(&self) -> Option<Box<V>> {
        if self.type_id()
            == std::any::TypeId::of::<<V as DynAnySized>::Static>()
        {
            let ptr = self as *const Self as *mut V;
            Some(unsafe { Box::from_raw(ptr) })
        } else {
            None
        }
    }
}*/
// use graphene_core::Node;
// pub trait DynAnyNode<'a>: 'a + Node<'a, ()> {
// type Static: Node<'static, ()> + ?Sized + 'static;
// fn type_id(&self) -> std::any::TypeId {
// std::any::TypeId::of::<Self::Static>()
// }
// }
// impl<'a, N: Node<'a, ()> + 'a> DynAnyNode<'a> for N
// where
// <N as Node<'a, ()>>::Output: DynAnySized<'a>,
// {
// type Static = dyn Node<
// 'static,
// (),
// Output = <<N as Node<'a, ()>>::Output as DynAnySized<'a>>::Static,
// > + 'static;
// }
// impl<'a, T: DynAnyNode<'a>> DynAny<'a> for T {
// type Static = <T as DynAnyNode<'a>>::Static;
// }
use std::any::TypeId;

pub trait NewAny<'a> {
    fn type_id(&self) -> TypeId;
}

/*impl<'a, T: DynAny<'a> + ?Sized> NewAny<'a> for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<<Self as DynAny<'a>>::Static>()
    }
}*/
impl<'a> NewAny<'a> for f32 {
    fn type_id(&self) -> TypeId {
        TypeId::of::<f32>()
    }
}
impl<'a, T: 'static> NewAny<'a> for &T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<&'static T>()
    }
}
pub fn downcast_ref<'a, V: DynAny<'a>>(i: &'a dyn NewAny<'a>) -> Option<&'a V> {
    if i.type_id() == std::any::TypeId::of::<<V as DynAny>::Static>() {
        // SAFETY: caller guarantees that T is the correct type
        let ptr = i as *const dyn NewAny<'a> as *const V;
        Some(unsafe { &*ptr })
    } else {
        None
    }
}

impl<'a> dyn NewAny<'a> {
    /*pub fn downcast_ref<'n, V>(a: &'n dyn NewAny<'n>) -> &'n V {
        let a = unsafe { &*(a as *const dyn NewAny<'n> as *const V) };
        a
    }*/
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
        impl<'a, $($($T: 'a + $crate::DynAnyClone<'a> + Sized,)*)?> $crate::DynAnyClone<'a> for $id $(<$($($l,)*)?$($T, )*>)?{
            type Static = $id$(<$($($s,)*)?$(<$T as $crate::DynAnyClone<'a>>::Static,)*>)?;
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
impl<'a, T: 'a + DynAnyClone<'a>> DynAnyClone<'a> for &'a T {
    type Static = &'static <T as DynAnyClone<'a>>::Static;
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

impl_type!(Option<T>,Result<T, E>,/*Cell<T>,UnsafeCell<T>,*/RefCell<T>,/*MaybeUninit<T>,*/
           Vec<T>, String, BTreeMap<K,V>,BTreeSet<V>, LinkedList<T>, VecDeque<T>,
           BinaryHeap<T>, ManuallyDrop<T>, PhantomData<T>, PhantomPinned,Empty<T>,
           Wrapping<T>, Duration, /*Once, Mutex<T>, RwLock<T>,*/  bool, f32, f64, char,
           //u8, AtomicU8, u16,AtomicU16, u32,AtomicU32, u64,AtomicU64, usize,AtomicUsize,
           //i8,AtomicI8, i16,AtomicI16, i32,AtomicI32, i64,AtomicI64, isize,AtomicIsize,
           u8, u16, u32, u64, usize,
           i8, i16, i32, i64, isize,
            i128, u128//, AtomicBool, AtomicPtr<T>
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
