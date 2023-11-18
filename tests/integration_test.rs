// #[macro_use]
extern crate class_scaffolding;

#[cfg(test)]
mod tests {
    use class_scaffolding::add_field;
    // #[derive(AnswerFn)]
    // struct Struct;

    #[test]
    fn test_add_fields() {
        #[add_field]
        #[derive(Debug, Clone)]
        struct Foo {}
        
        let bar = Foo { a: "lorem ipsum".to_string()};
        assert_eq!(format!("{:?}", bar), "{}");;
    }

    // fn test_macro3() {
    //     assert_eq!(42, answer());
    // }

    // // #[test]
    // // fn test_macro2() {
    // //     #[derive(HelperAttr)]
    // //     struct MyObj {
    // //         #[helper]
    // //         field: (),
    // //     }
    // //     let s = MyObj();
    // // }
}
