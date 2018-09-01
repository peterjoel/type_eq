extern crate type_eq;
use type_eq::{Constrain, TypeEq}; 

struct Pair<A, B> {
    a: A,
    b: B,
}

// silly example: could just have implemented for Pair<A, A>
impl<A: Copy, B: Copy> Pair<A, B> {
    // Constrain A = B without changing the enclosing impl
    fn to_vec(&self) -> Vec<A> where Constrain: TypeEq<B, A> {
        vec![self.a, Constrain.as_eq(self.b)]
    }
}

fn main() {
    let p = Pair { a: 1, b: 2 };
    let v = p.to_vec();
    assert_eq!(vec![1, 2], v);
}
