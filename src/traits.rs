pub trait MutPtrWrapper <T> {
    fn ptr (&self) -> T;
}

#[macro_escape]
macro_rules! generate_mut_ptr_wrapper {
    ($wrapper_name:ident : $ptr_type:ty; $($default_trait:ident),*) => {

        pub struct $wrapper_name {
            pub ptr : $ptr_type
        }

        impl MutPtrWrapper<$ptr_type> for $wrapper_name  {
            #[inline]
            fn ptr (&self) -> $ptr_type {
                self.ptr
            }
        }

        $(impl $default_trait for $wrapper_name {})*
    };
}
