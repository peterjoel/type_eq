/// A convenience struct for applying implementing `TypeEq` trait. Since the trait
/// connects two types, this allows neither to be Self, and prevents issues with
/// asymmetry.
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

