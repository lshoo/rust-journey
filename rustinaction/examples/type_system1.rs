
//! https://towardsdev.com/exploiting-the-rusts-type-system-to-avoid-runtime-errors-37193b2aafc6

use std::marker::PhantomData;

#[derive(Clone)]
pub struct SafeVec<E, S> {
    raw_vec: Vec<E>,
    state: PhantomData<S>
}

pub struct Sorted;
pub struct UnSorted;

impl<E, S> SafeVec<E, S> {
    pub fn len(&self) -> usize {
        self.raw_vec.len()
    }

    pub fn pop(&mut self) -> Option<E> {
        self.raw_vec.pop()
    }
}
// methods that can be call by unsorted SafeVecs
impl <E> SafeVec<E, UnSorted> where E: PartialEq + PartialOrd + Ord + Eq {
    pub fn push(&mut self, element: E) {
        self.raw_vec.push(element);
    }

    pub fn sort(mut self) -> SafeVec<E, Sorted> {
        self.raw_vec.sort();
        SafeVec { 
            raw_vec: self.raw_vec,
            state: PhantomData::default(),
        }
    }
}

impl <E> SafeVec<E, Sorted> where E: PartialEq + PartialOrd + Ord + Eq {
    pub fn new() -> SafeVec<E, Sorted> {
        SafeVec { 
            raw_vec: Vec::new(),
            state: PhantomData::default(),
        }
    }

    pub fn relaxed(self) -> SafeVec<E, UnSorted> {
        SafeVec { 
            raw_vec: self.raw_vec,
            state: PhantomData::default(),
        }
    }

    pub fn search(&self, element: &E) -> Option<usize> {
        self.raw_vec.binary_search(element).ok()
    }
     
}
fn main() {
    let mut v = SafeVec::new().relaxed();
    v.push(1);
    v.push(3);
    v.push(2);

    let v = v.sort();   // Must sort first
    let index = v.search(&2);
    println!("The index is {:?}", index);
}