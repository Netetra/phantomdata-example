use std::marker::PhantomData;

struct A;
struct B;

struct Hoge<T> {
    marker: PhantomData<T>,
}

impl<T> Hoge<T> {
    fn new() -> Hoge<T> {
        Hoge {
            marker: PhantomData,
        }
    }
}

impl Hoge<A> {
    fn to_b(self) -> Hoge<B> {
        Hoge {
            marker: PhantomData,
        }
    }
}

impl Hoge<B> {
    fn to_a(self) -> Hoge<A> {
        Hoge {
            marker: PhantomData,
        }
    }
}

fn main() {
    let hoge = Hoge::<A>::new();
    let hoge = hoge.to_b();
    // let hoge = hoge.to_b(); // this is error
    let _hoge = hoge.to_a();
}
