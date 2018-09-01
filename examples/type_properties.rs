extern crate type_eq;
use type_eq::{Constrain, Has, TypeEq}; 

struct X;
struct Y;

struct Point<T> { x: T, y: T }

impl<T: Copy> Has<X> for Point<T> {
    type Val = T;
    fn get<P>(&self) -> &Self::Val
    where
        Constrain: TypeEq<X, P> 
    {
        &self.x
    }
}

impl<T: Copy> Has<Y> for Point<T> {
    type Val = T;
    fn get<P>(&self) -> &Self::Val
    where
        Constrain: TypeEq<Y, P> 
    {
        &self.y
    }
}

fn main() {
    let p = Point { x: 3, y: 5 };
    assert_eq!(&3, p.get::<X>());
    assert_eq!(&5, p.get::<Y>());
}