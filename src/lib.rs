// "manu_mock" module does work
//
//mod manu_mock;

mod auto_mock;

#[cfg(test)]
mod tests {
    #[double]
    use crate::auto_mock::mockable::S;
    use mockall_double::double;
    //use crate::manu_mock::mockable::S;

    #[test]
    fn it_works() {
        fn with_s(s: S) {
            s.f();
        }
    }
}
