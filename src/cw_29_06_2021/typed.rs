pub mod type_name {
    pub trait Typed {
        fn type_name(&self) -> &'static str;
    }
    impl<T> Typed for T {
        fn type_name(&self) -> &'static str {
            std::any::type_name::<T>()
        }
    }
}
