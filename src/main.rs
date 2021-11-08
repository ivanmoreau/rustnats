use std::{marker::PhantomData};

trait Nat {
    fn get() -> i32;
}

struct Z;

struct S<N: Nat>(PhantomData<N>);

impl Nat for Z {
    fn get() -> i32 {
        0
    }
}
impl<N> Nat for S<N> where N: Nat {
    fn get () -> i32 {
        1 + N::get()
    }
}

// _ + _ : Nat -> Nat -> Nat
trait Plus<N: Nat>: Nat {
    type Result: Nat;
}
// Z + n = n
impl<N: Nat> Plus<N> for Z {
    type Result = N;
}
// (S m) + n = S (m + n)
impl<N: Nat, M: Nat> Plus<N> for S<M> 
where M: Plus<N> {
    type Result = S<M::Result>;
}


/*
_*_ : Nat -> Nat -> Nat
0 * n = 0
(suc m) * n = n + (m * n)
*/

// _*_ : Nat -> Nat -> Nat
trait Mul<N: Nat>: Nat {
    type Result: Nat;
}

// 0 * n = 0
impl<N: Nat> Mul<N> for Z {
    type Result = Z;
}

//(suc m) * n = n + (m * n)
impl<N: Nat, M: Nat> Mul<N> for S<M>
where M: Mul<N>,
N : Mul <M> + Plus<<N as Mul<M>>::Result> {
    type Result = <N as Plus< <N as Mul<M>>::Result >>::Result;
}


// _^_ : Nat -> Nat -> Nat
trait Exp<N: Nat>: Nat {
    type Result: Nat;
}

// n ^ 0 = 1
impl<N: Nat> Exp<Z> for N {
    type Result = S<Z>;
}

// n ^ suc(m) = n * (n ^ m)
impl<N: Nat, M: Nat> Exp<S<M>> for N 
where N: Exp<M> + Mul<<N as Exp<M>>::Result> {
    type Result = <N as Mul< <N as Exp<M>>::Result >>::Result;
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let _x: S<S<S<Z>>> = S(PhantomData);
    let _y: S<Z> = S(PhantomData);
    let f: <S<S<S<Z>>> as Exp<S<S<Z>>>>::Result = S(PhantomData);
    let sd = f;
    let _cd: <S<Z> as Plus<S<Z>>>::Result = S(PhantomData);
    print_type_of(&sd);}