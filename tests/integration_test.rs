// #[macro_use]
extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_core::*;
    use scaffolding_macros::*;

    #[as_entity]
    #[derive(Debug, Clone, Scaffolding)]
    struct MyEntity {
        b: bool
    }

    #[test]
    fn test_entity() {
        let entity = MyEntity {id: "lorem ipsum".to_string(), b: true};
        entity.hello();
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
