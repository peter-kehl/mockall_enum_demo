pub mod mockable {
    #[cfg(test)]
    use mockall::automock;

    pub struct S {}

    #[cfg_attr(test, automock)]
    impl S {
        pub fn f(self) {}
    }
}
