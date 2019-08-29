#![recursion_limit = "128"]

#[macro_use]
extern crate cpp;

mod bindings;
pub mod memory;
pub mod string;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unittest_layout() {
        use bindings::root::rust;
        use bindings::root::std as stl;
        use std::mem::{align_of, size_of};

        assert_eq!(size_of::<string::String>(), size_of::<stl::string>());
        assert_eq!(align_of::<string::String>(), align_of::<stl::string>());

        assert_eq!(
            size_of::<vector::VectorOfI32>(),
            size_of::<rust::vector_of_int32_t>()
        );
        assert_eq!(
            align_of::<vector::VectorOfI32>(),
            align_of::<rust::vector_of_int32_t>()
        );

        assert_eq!(
            size_of::<vector::VectorOfUniquePtr<string::String>>(),
            size_of::<rust::vector_of_dummy_unique_ptr>()
        );
        assert_eq!(
            align_of::<vector::VectorOfUniquePtr<string::String>>(),
            align_of::<rust::vector_of_dummy_unique_ptr>()
        );
    }
}
