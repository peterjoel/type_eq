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
///     // be the same type as A and B, this can allow the caller a more convenient way of 
///     // determining those types 
///     fn infer_a<T>(&self) where Constrain: TypeEq<A, T> + TypeEq<B, T> { }
/// }
/// ```
pub struct Constrain;

/// A trait that determines that two type parameters are the same type.
/// There is only one implementation of this trait, `Constrain`, which ensures that all types 
/// satisfy the condition of being equal to themselves. There should not be a reason to implement 
/// this for other types, which is why it is marked as `unsafe`.
pub unsafe trait TypeEq<A, B> {
    fn as_eq(&self, a: A) -> B;
}

unsafe impl<A> TypeEq<A, A> for Constrain {
    fn as_eq(&self, a: A) -> A { a }
}

pub unsafe trait Outlives<'a: 'a, 'b, A: 'a, B: 'b> {
    fn proof<'r>(&self, a: &'r A) -> &'r B;
}

unsafe impl<'a: 'a, 'b, A: 'a, B: 'b> Outlives<'a, 'b, A, B> for Constrain {
    fn proof<'r>(&self, _: &'r A) -> &'r B { unimplemented!() }
}

/// Type-level properties. This trait can be used to associate marker types with properties of
/// the implementating type.
/// 
/// # Examples
/// ```
/// use type_eq::{Constrain, Has, TypeEq};
/// 
/// // marker type for the "name" property
/// struct Name;
/// 
/// struct Foo;
/// 
/// impl Has<Name> for Foo {
///     type Val = &'static str;
///     fn get<P>(&self) -> Self::Val 
///     where
///         Constrain: TypeEq<Name, P>,
///     {
///         "Foo"
///     }
/// }
/// 
/// assert_eq!("Foo", Foo.get::<Name>());
/// 
/// ```
/// 
pub trait Has<Prop> {
    type Val: Sized;
    fn get<P>(&self) -> Self::Val
    where
        Constrain: TypeEq<Prop, P>;
}



#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let s1 = String::from("s1");
        {
            let s2 = String::from("s2");


        }
    }
}