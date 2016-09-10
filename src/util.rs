// Copyright 2016 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashSet, hash_set};
use std::borrow::Borrow;
use std::hash::Hash;
use float::*;

pub use self::one_or_two::OneOrTwo;

pub fn quad_root_ascending(a: R64, b: R64, c: R64) -> Option<N64> {
    let determinant = b * b - a * c * 4.0;
    if determinant <= 0.0 {
        None
    } else if b >= 0.0 {
        Some(N64::from(c * 2.0) / N64::from(-b - determinant.sqrt()))
    } else {
        Some(N64::from(-b + determinant.sqrt()) / N64::from(a * 2.0))
    }
}

const MIN_TIGHT_SET_CAPACITY: usize = 4;

#[derive(Clone)]
pub struct TightSet<T: Hash + Eq> {
    set: HashSet<T>
}

impl <T: Hash + Eq> TightSet<T> {
    pub fn new() -> TightSet<T> {
        TightSet { set : HashSet::with_capacity(MIN_TIGHT_SET_CAPACITY) }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool 
        where T: Borrow<Q>, Q: Hash + Eq
    {
        self.set.contains(value)
    }
    
    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool 
        where T: Borrow<Q>, Q: Hash + Eq
    {
        let success = self.set.remove(value);
        if success && self.set.capacity() > MIN_TIGHT_SET_CAPACITY && self.set.capacity() >= self.set.len()*4 {
            self.set.shrink_to_fit();
        }
        success
    }

    pub fn iter(&self) -> hash_set::Iter<T> {
        self.set.iter()
    }
    
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
    
    pub fn clear(&mut self) {
        if self.set.capacity() <= MIN_TIGHT_SET_CAPACITY {
            self.set.clear();
        } else {
            self.set = HashSet::with_capacity(MIN_TIGHT_SET_CAPACITY);
        }
    }
}

mod one_or_two {
    pub enum OneOrTwo<T: Copy + Eq> {
        One(T),
        Two(T, T)
    }
    
    impl <T: Copy + Eq> OneOrTwo<T> {
        pub fn other_id(self, id: T) -> Option<T> {
            match self {
                OneOrTwo::One(id_1) if id_1 == id => None,
                OneOrTwo::Two(id_1, id_2) | OneOrTwo::Two(id_2, id_1) if id_1 == id => Some(id_2),
                _ => panic!("illegal state")
            }
        }
        
        pub fn iter(self) -> Iter<T> {
            Iter { one_or_two : self, index : 0 }
        }
    }
    
    pub struct Iter<T: Copy + Eq> {
        one_or_two: OneOrTwo<T>,
        index: u8
    }
    
    impl <T: Copy + Eq> Iterator for Iter<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            let result = match (&self.one_or_two, self.index) {
                (&OneOrTwo::One(val), 0) | (&OneOrTwo::Two(val, _), 0) | (&OneOrTwo::Two(_, val), 1) => Some(val),
                _ => None
            };
            if result.is_some() { self.index += 1 }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use float::*;
    use super::*;
    
    #[test]
    fn test_quad_root_ascending() {
        assert!((quad_root_ascending(r64(1e-14), r64(2.0), r64(-1.0)).unwrap() - 0.5).abs() < 1e-7);
        assert!((quad_root_ascending(r64(0.0), r64(2.0), r64(-1.0)).unwrap() - 0.5).abs() < 1e-7);
        assert!((quad_root_ascending(r64(100.0), r64(-1.0), r64(-1e-16)).unwrap() - 0.01).abs() < 1e-7);
        assert!(quad_root_ascending(r64(0.0), r64(-2.0), r64(1.0)).unwrap().is_infinite());
        assert!(quad_root_ascending(r64(-3.0), r64(0.0), r64(-1.0)).is_none());
        assert!(quad_root_ascending(r64(1.0), r64(1.0), r64(1.0)).is_none());
    }
}