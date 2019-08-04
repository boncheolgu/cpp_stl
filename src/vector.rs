use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice;

use libc::size_t;

use crate::bindings::root::rust::*;

cpp! {{
    #include <memory>
    #include <vector>

    #include "wrapper.hpp"

    struct name {
        std::string first_name;
        std::string last_name;
    };

    struct superclass {
        int32_t dummy;
    };

    struct subclass : superclass {
        int32_t value;
        std::string desc;

        subclass(int32_t v, const char* s): value(v), desc(s) {}
    };

    struct struct_with_vectors {
        std::vector<int32_t> ids;
        std::vector<std::unique_ptr<name>> names;
        std::vector<int32_t> ages;
        std::vector<std::unique_ptr<std::string>> cities;
        std::vector<std::unique_ptr<subclass>> derived;
    };
}}

#[repr(C)]
#[derive(Debug)]
pub struct VectorOfBool(vector_of_bool);

#[repr(C)]
#[derive(Debug)]
pub struct VectorOfU8(vector_of_uint8_t);

#[repr(C)]
pub struct VectorOfI32(vector_of_int32_t);

impl Deref for VectorOfI32 {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for VectorOfI32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl Index<usize> for VectorOfI32 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for VectorOfI32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl<'a> IntoIterator for &'a VectorOfI32 {
    type Item = &'a i32;
    type IntoIter = slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut VectorOfI32 {
    type Item = &'a mut i32;
    type IntoIter = slice::IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl fmt::Debug for VectorOfI32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl VectorOfI32 {
    pub fn size(&self) -> size_t {
        unsafe {
            cpp!([self as "const std::vector<int32_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    pub fn as_slice(&self) -> &[i32] {
        let ptr = unsafe {
            cpp!([self as "const std::vector<int32_t>*"]
                  -> *const i32 as "const int32_t*" {
                return self->data();
            })
        };
        unsafe { slice::from_raw_parts(ptr, self.size()) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [i32] {
        let ptr = unsafe {
            cpp!([self as "std::vector<int32_t>*"]
                  -> *mut i32 as "int32_t*" {
                return self->data();
            })
        };
        unsafe { slice::from_raw_parts_mut(ptr, self.size()) }
    }

    pub fn resize(&mut self, new_len: usize, value: i32) {
        let new_len = new_len as size_t;
        unsafe {
            cpp!([self as "std::vector<int32_t>*", new_len as "size_t", value as "int32_t"] {
                self->resize(new_len, value);
            })
        }
    }

    pub fn push_back(&mut self, v: i32) {
        unsafe {
            cpp!([self as "std::vector<int32_t>*", v as "int32_t"] {
                self->push_back(v);
            })
        }
    }

    pub fn pop_back(&mut self) {
        unsafe {
            cpp!([self as "std::vector<int32_t>*"] {
                self->pop_back();
            })
        }
    }

    pub fn back(&self) -> &i32 {
        unsafe {
            cpp!([self as "std::vector<int32_t>*"] -> &i32 as "const int32_t*" {
                return &self->back();
            })
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            cpp!([self as "std::vector<int32_t>*"] {
                self->clear();
            })
        }
    }

    pub fn erase(&mut self, pos: usize) {
        let pos = pos as size_t;
        unsafe {
            cpp!([self as "std::vector<int32_t>*", pos as "size_t"] {
                self->erase(self->begin() + pos);
            })
        }
    }

    pub fn assign<I: IntoIterator<Item = i32>>(&mut self, vs: I) {
        self.clear();
        for v in vs {
            self.push_back(v);
        }
    }
}

pub struct Iter<'a, T> {
    vector: &'a VectorOfUniquePtr<T>,
    index: usize,
}

#[repr(C)]
pub struct VectorOfUniquePtr<T>(vector_of_unique_ptr, PhantomData<T>);

impl<T> Index<usize> for VectorOfUniquePtr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let index = index as size_t;
        let ptr = unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<void>>*", index as "size_t"]
                  -> *const c_void as "const void*" {
                return (*self)[index].get();
            }) as *const Self::Output
        };
        unsafe { &*ptr as &Self::Output }
    }
}

impl<T> IndexMut<usize> for VectorOfUniquePtr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index as size_t;
        let ptr = unsafe {
            cpp!([self as "std::vector<std::unique_ptr<void>>*", index as "size_t"]
                  -> *mut c_void as "void*" {
                return (*self)[index].get();
            }) as *mut Self::Output
        };
        unsafe { &mut *ptr as &mut Self::Output }
    }
}

impl<T> fmt::Debug for VectorOfUniquePtr<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, T> IntoIterator for &'a VectorOfUniquePtr<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.vector.size() {
            return None;
        }

        self.index += 1;
        Some(&self.vector[self.index - 1])
    }
}

impl<T> VectorOfUniquePtr<T> {
    pub fn size(&self) -> size_t {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<void>>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            vector: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string::String as StlString;

    #[repr(C)]
    struct Name {
        first_name: StlString,
        last_name: StlString,
    }

    #[repr(C)]
    struct Subclass {
        parent: i32,
        value: i32,
        desc: StlString,
    }

    #[repr(C)]
    struct StructWithVectors {
        ids: VectorOfI32,
        names: VectorOfUniquePtr<Name>,
        ages: VectorOfI32,
        cities: VectorOfUniquePtr<StlString>,
        derived: VectorOfUniquePtr<Subclass>,
    }

    #[test]
    fn unittest_vector() {
        use std::mem;
        assert_eq!(
            mem::size_of::<VectorOfI32>(),
            mem::size_of::<vector_of_int32_t>()
        );

        assert_eq!(
            mem::size_of::<VectorOfUniquePtr<i32>>(),
            mem::size_of::<vector_of_unique_ptr>()
        );

        let x = unsafe {
            cpp!([] -> &mut StructWithVectors as "struct_with_vectors*" {
                static struct_with_vectors x{
                    {10, 18, 23, 31}, {}, {7, 10, 42}, {}, {}
                };
                x.names.emplace_back(new name {
                    std::string("boncheol"), std::string("gu")
                });
                x.names.emplace_back(new name {
                    std::string("bora"), std::string("hong")
                });
                x.cities.emplace_back(new std::string("seoungnam"));
                x.derived.emplace_back(new subclass(99, "derived"));
                return &x;
            })
        };

        assert_eq!(x.ids.size(), 4);
        assert_eq!(x.names.size(), 2);
        assert_eq!(x.ages.size(), 3);
        assert_eq!(x.cities.size(), 1);
        assert_eq!(x.ids.as_slice(), &[10i32, 18, 23, 31]);
        assert_eq!(
            x.names
                .iter()
                .map(|name| name.first_name.c_str().to_string_lossy())
                .collect::<Vec<_>>(),
            vec!["boncheol", "bora"]
        );
        assert_eq!(x.names[0].first_name.c_str().to_string_lossy(), "boncheol");
        assert_eq!(x.names[0].last_name.c_str().to_string_lossy(), "gu");
        assert_eq!(x.names[1].first_name.c_str().to_string_lossy(), "bora");
        assert_eq!(x.names[1].last_name.c_str().to_string_lossy(), "hong");
        assert_eq!(x.ages[0], 7);
        assert_eq!(x.ages[1], 10);
        assert_eq!(x.ages[2], 42);
        assert_eq!(x.cities[0].c_str().to_string_lossy(), "seoungnam");
        assert_eq!(x.derived[0].value, 99);
        assert_eq!(x.derived[0].desc.c_str().to_string_lossy(), "derived");

        x.ids[0] = 20;
        x.ids[2] = 43;
        assert_eq!(x.ids.as_slice(), &[20i32, 18, 43, 31]);

        x.ids.push_back(9);
        assert_eq!(x.ids.size(), 5);
        assert_eq!(x.ids.back(), &9);

        x.ages.assign(vec![8, 7]);
        assert_eq!(x.ages.as_slice(), &[8, 7]);
    }
}
