pub mod mockable {
    #[cfg(test)]
    use mockall::mock;

    pub struct S {}

    #[cfg(not(test))]
    impl S {
        pub fn f(self) {}
    }
    #[cfg(test)]
    mock! {
        pub S {
            pub fn f(self);
        }
    }
}
