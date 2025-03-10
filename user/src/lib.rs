#[cfg(test)]
mod test {
    use builder::*;

    #[test]
    fn test_vanilla() {
        #[builder]
        struct MyStruct {
            a: i32,
            b: String,
            c: f64,
        }

        let s = MyStruct::builder()
            .a(10)
            .b(String::from("Hello"))
            .c(4.2)
            .build();
        assert_eq!(s.a, 10);
        assert_eq!(s.b, String::from("Hello"));
        assert_eq!(s.c, 4.2);
    }

    #[test]
    fn test_custom_name() {
        #[builder(name=CustomBuilder)]
        struct MyStruct {
            a: i32,
            b: String,
            c: f64,
        }

        let s = CustomBuilder::new()
            .a(10)
            .b(String::from("Hello"))
            .c(4.2)
            .build();
        assert_eq!(s.a, 10);
        assert_eq!(s.b, String::from("Hello"));
        assert_eq!(s.c, 4.2);
    }

    #[test]
    fn test_hidden() {
        #[builder]
        struct MyStruct {
            #[build(hidden)]
            a: i32,
            b: String,
            c: f64,
        }

        let s = MyStruct::builder().b(String::from("Hello")).c(4.2).build();
        assert_eq!(s.a, 0);
        assert_eq!(s.b, String::from("Hello"));
        assert_eq!(s.c, 4.2);
    }

    #[test]
    fn test_hidden_custom_default() {
        #[builder]
        struct MyStruct {
            #[build(hidden, default_value = 42)]
            a: i32,
            #[build(hidden, default_value=String::from("Hi"))]
            b: String,
            c: f64,
        }

        let s = MyStruct::builder().c(4.2).build();
        assert_eq!(s.a, 42);
        assert_eq!(s.b, String::from("Hi"));
        assert_eq!(s.c, 4.2);
    }

    #[test]
    fn test_custom_default_overriden() {
        #[builder]
        struct MyStruct {
            #[build(default_value = 42)]
            a: i32,
            b: String,
            c: f64,
        }

        let s = MyStruct::builder()
            .a(35)
            .b(String::from("Hello"))
            .c(4.2)
            .build();
        assert_eq!(s.a, 35);
        assert_eq!(s.b, String::from("Hello"));
        assert_eq!(s.c, 4.2);
    }

    #[test]
    fn test_custom_type_no_default() {
        struct MyType {
            custom: i32,
        }

        impl MyType {
            fn new(custom: i32) -> Self {
                Self { custom }
            }
        }

        #[builder]
        struct MyStruct {
            #[build(default_value=MyType::new(12))]
            a: MyType,
        }

        let s = MyStruct::builder().build();
        assert_eq!(s.a.custom, 12);
    }

    #[test]
    fn test_custom_setter() {
        #[builder]
        struct MyStruct {
            #[build(setter=b)]
            a: i32,
            #[build(setter=a)]
            b: i32,
        }

        let s = MyStruct::builder().b(42).a(10).build();
        assert_eq!(s.a, 42);
        assert_eq!(s.b, 10);
    }
}
