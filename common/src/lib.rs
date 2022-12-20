mod runner;
mod net;

extern crate core;

#[cfg(test)]
mod tests {
    use std::any::Any;
    use syn::parse_macro_input;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
