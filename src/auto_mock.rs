pub mod mockable {
    #[cfg(test)]
    use mockall::automock;

    #[cfg_attr(test, automock)]
    pub struct S {}

    impl S {
        pub fn f(self) {}
    }
}
