/// A convenience struct for applying implementing `TypeEq` trait. Since the trait
/// connects two types, this allows neither to be Self, and prevents issues with
/// asymmetry.
/// 
/// # Examples
/// 
/// ```
/// # use std::marker::PhantomData;
/// use type_eq::{Constrain, TypeEq};
/// struct Example<A, B> { 
///     # _marker: PhantomData<(A, B)>,
///     // ... 
/// }
/// 
/// impl<A, B> Example<A, B> {
/// 
///     // This method is only defined when A and B are the same type
///     fn a_eq_b(&self) where Constrain: TypeEq<A, B> { }
/// 
///     // This method is only defined when it's called with a parameter T which is
///     // the same type as A
///     fn a_eq_t<T>(&self, _t: T) where Constrain: TypeEq<A, T> { }
/// 
///     // This method must be called with an explicit type. Since T is contrained to
///     // be the same type as A and B, this can allow the caller a more conveniet way of 
///     // determining those types 
///     fn infer_a<T>(&self) where Constrain: TypeEq<A, T> + TypeEq<B, T> { }
/// }
/// ```
pub struct Constrain;

/// There are no situations where you should implement this trait except for the one
/// impl for `Constrain`, which connects all 
pub unsafe trait TypeEq<A, B> {
    fn as_eq(&self, a: A) -> B;
}

unsafe impl<A> TypeEq<A, A> for Constrain {
    fn as_eq(&self, a: A) -> A { a }
}

/// Type-level properties
pub trait Has<Prop> {
    type Val;
    fn get<P>(&self) -> &Self::Val
    where
        Constrain: TypeEq<Prop, P>;
}

