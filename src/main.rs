// type Lambda = dyn Fn(()) -> Option2;

// enum Option2 {
//     Some(Box<Lambda>),
//     None,
// }

// type Lambda = dyn Fn(Lambda) -> Lambda;

// type Lambda = dyn Fn(()) -> Option<Box<Lambda>>;

use std::fmt::{Debug, Formatter};
use std::ops::{BitOr};
use std::ptr;
use std::rc::Rc;

#[derive(Clone)]
struct Lambda(Rc<dyn Fn(Lambda) -> Lambda>);

impl BitOr for Lambda {
    type Output = Lambda;

    fn bitor(self, rhs: Self) -> Self::Output {
        let Lambda(func) = self;

        func(rhs)
    }
}

impl BitOr for &Lambda {
    type Output = Lambda;

    fn bitor(self, rhs: Self) -> Self::Output {
        let Lambda(func) = self;

        func(rhs.clone())
    }
}

impl Debug for Lambda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambda {:?}", self.id())
    }
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0.as_ref(), other.0.as_ref())
    }
}

impl Lambda {
    fn id(&self) -> *const dyn Fn(Lambda) -> Lambda {

        std::ptr::addr_of!(*self.0)

    }
}

fn main() {

    let identity = Lambda(Rc::new(|x| x));

    dbg!(identity.id());

    let zero_identity = identity.clone();

    dbg!(zero_identity.id());

    let zero = Lambda(Rc::new(move |f| zero_identity.clone()));

    dbg!(zero.id());

    let one = Lambda(Rc::new(|f| Lambda(Rc::new(move |x| f.clone() | x))));

    let truel = Lambda(Rc::new(|x| Lambda(Rc::new(move |y| x.clone()))));

    let falsel = zero.clone();

    let and = Lambda(Rc::new(|p| Lambda(Rc::new(move |q| p.clone() | q.clone() | p.clone()))));

    assert_eq!(and.clone() | truel.clone() | falsel.clone(), falsel.clone());
    assert_eq!(and.clone() | truel.clone() | truel.clone(), truel.clone());
    assert_eq!(and.clone() | falsel.clone() | falsel.clone(), falsel.clone());
    assert_eq!(and.clone() | falsel.clone() | truel.clone(), falsel.clone());

    // Lambda(Rc::new(||))

    let or = Lambda(Rc::new(|p| Lambda(Rc::new(move |q| p.clone() | p.clone() | q.clone()))));

    assert_eq!(or.clone() | truel.clone() | falsel.clone(), falsel.clone());
    assert_eq!(or.clone() | truel.clone() | truel.clone(), truel.clone());
    assert_eq!(or.clone() | falsel.clone() | falsel.clone(), falsel.clone());
    assert_eq!(or.clone() | falsel.clone() | truel.clone(), falsel.clone());

    // dbg!(zero.clone() | one.clone());
    // dbg!(one.clone() | identity.clone());
    // dbg!(one.clone() | identity.clone());

}
