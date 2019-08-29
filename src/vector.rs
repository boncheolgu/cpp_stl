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

pub trait BasicVector {
    type Item;

    fn get_ptr(&self) -> *const Self::Item;

    fn get_mut_ptr(&self) -> *mut Self::Item;

    fn size(&self) -> usize;

    fn push_back(&mut self, v: Self::Item);

    fn pop_back(&mut self);

    fn erase(&mut self, offset: usize, size: usize);
}

pub trait Vector: BasicVector
where
    <Self as BasicVector>::Item: Clone,
{
    fn as_slice(&self) -> &[Self::Item];

    fn as_mut_slice(&mut self) -> &mut [Self::Item];

    fn resize(&mut self, new_len: usize, value: Self::Item);

    fn assign<I: IntoIterator<Item = Self::Item>>(&mut self, vs: I);

    fn clear(&mut self);

    fn retain(&mut self, pred: fn(usize, &Self::Item) -> bool);
}

impl<T> Vector for T
where
    T: BasicVector,
    <T as BasicVector>::Item: Clone,
{
    fn as_slice(&self) -> &[Self::Item] {
        unsafe { slice::from_raw_parts(self.get_ptr(), self.size()) }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        unsafe { slice::from_raw_parts_mut(self.get_mut_ptr(), self.size()) }
    }

    fn resize(&mut self, new_len: usize, value: Self::Item) {
        while self.size() < new_len {
            self.push_back(value.clone());
        }

        while self.size() > new_len {
            self.pop_back();
        }
    }

    fn assign<I: IntoIterator<Item = Self::Item>>(&mut self, vs: I) {
        self.clear();
        for v in vs {
            self.push_back(v);
        }
    }

    fn clear(&mut self) {
        while self.size() != 0 {
            self.pop_back();
        }
    }

    fn retain(&mut self, pred: fn(usize, &Self::Item) -> bool) {
        let removed: Vec<_> = self
            .as_slice()
            .iter()
            .enumerate()
            .filter_map(|(i, op)| if pred(i, op) { None } else { Some(i) })
            .collect();

        for i in removed.into_iter().rev() {
            self.erase(i, 1);
        }
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl fmt::Debug for $t
        where
            <$t as BasicVector>::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.as_slice().iter()).finish()
            }
        }

        impl Deref for $t {
            type Target = [<$t as BasicVector>::Item];

            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut_slice()
            }
        }

        impl Index<usize> for $t {
            type Output = <$t as BasicVector>::Item;

            fn index(&self, index: usize) -> &Self::Output {
                &self.as_slice()[index]
            }
        }

        impl IndexMut<usize> for $t {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut_slice()[index]
            }
        }

        impl<'a> IntoIterator for &'a $t {
            type Item = &'a <$t as BasicVector>::Item;
            type IntoIter = slice::Iter<'a, <$t as BasicVector>::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $t {
            type Item = &'a mut <$t as BasicVector>::Item;
            type IntoIter = slice::IterMut<'a, <$t as BasicVector>::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }
    )*)
}

add_impl!(VectorOfU8 VectorOfI32 VectorOfI64 VectorOfF32);

#[repr(C)]
#[derive(Debug)]
pub struct VectorOfBool(vector_of_bool);

#[repr(C)]
pub struct VectorOfU8(vector_of_uint8_t);

impl BasicVector for VectorOfU8 {
    type Item = u8;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<uint8_t>*"]
                  -> *const u8 as "const uint8_t*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<uint8_t>*"]
                  -> *mut u8 as "const uint8_t*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<uint8_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<uint8_t>*", v as "uint8_t"] {
                self->push_back(v);
            })
        }
    }

    fn pop_back(&mut self) {
        unsafe {
            cpp!([self as "std::vector<uint8_t>*"] {
                self->pop_back();
            })
        }
    }

    fn erase(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<uint8_t>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

#[repr(C)]
pub struct VectorOfI32(vector_of_int32_t);

impl BasicVector for VectorOfI32 {
    type Item = i32;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<int32_t>*"]
                  -> *const i32 as "const int32_t*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<int32_t>*"]
                  -> *mut i32 as "const int32_t*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<int32_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<int32_t>*", v as "int32_t"] {
                self->push_back(v);
            })
        }
    }

    fn pop_back(&mut self) {
        unsafe {
            cpp!([self as "std::vector<int32_t>*"] {
                self->pop_back();
            })
        }
    }

    fn erase(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<int32_t>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

#[repr(C)]
pub struct VectorOfI64(vector_of_int64_t);

impl BasicVector for VectorOfI64 {
    type Item = i64;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<int64_t>*"]
                  -> *const i64 as "const int64_t*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<int64_t>*"]
                  -> *mut i64 as "const int64_t*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<int64_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<int64_t>*", v as "int64_t"] {
                self->push_back(v);
            })
        }
    }

    fn pop_back(&mut self) {
        unsafe {
            cpp!([self as "std::vector<int64_t>*"] {
                self->pop_back();
            })
        }
    }

    fn erase(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<int64_t>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

#[repr(C)]
pub struct VectorOfF32(vector_of_float);

impl BasicVector for VectorOfF32 {
    type Item = f32;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<float>*"]
                  -> *const f32 as "const float*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<float>*"]
                  -> *mut f32 as "const float*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<float>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<float>*", v as "float"] {
                self->push_back(v);
            })
        }
    }

    fn pop_back(&mut self) {
        unsafe {
            cpp!([self as "std::vector<float>*"] {
                self->pop_back();
            })
        }
    }

    fn erase(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<float>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

pub struct Iter<'a, T> {
    vector: &'a VectorOfUniquePtr<T>,
    index: usize,
}

#[repr(C)]
pub struct VectorOfUniquePtr<T>(vector_of_dummy_unique_ptr, PhantomData<T>);

impl<T> Index<usize> for VectorOfUniquePtr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let index = index as size_t;
        unsafe {
            let ptr = cpp!([self as "const std::vector<std::unique_ptr<void>>*", index as "size_t"]
                  -> *const c_void as "const void*" {
                return (*self)[index].get();
            }) as *const Self::Output;

            ptr.as_ref().unwrap()
        }
    }
}

impl<T> IndexMut<usize> for VectorOfUniquePtr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index as size_t;
        unsafe {
            let ptr = cpp!([self as "std::vector<std::unique_ptr<void>>*", index as "size_t"]
                  -> *mut c_void as "void*" {
                return (*self)[index].get();
            }) as *mut Self::Output;

            ptr.as_mut().unwrap()
        }
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
    pub fn size(&self) -> usize {
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
            mem::size_of::<vector_of_dummy_unique_ptr>()
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
        assert_eq!(x.ids.last().unwrap(), &9);

        x.ages.assign(vec![8, 7]);
        assert_eq!(x.ages.as_slice(), &[8, 7]);
    }
}
