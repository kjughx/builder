/// ```compile_fail(E0599)
///
/// #[builder::builder]
/// struct MyStruct {
///     a: i32,
///     b: String,
///     c: f64,
/// }
///
/// let _ = MyStruct::builder()
///     .a(10)
///     .b(String::from("Hello"))
///     .c(4.2)
///     .build();
/// ```
// There's no built-in support for checking compilation errors of actual tests,
// but there is for doc tests.
struct _Docs;

#[cfg(test)]
mod test {
    #[builder(name = CustomBuilder, a = b)]
    #[derive(Default, Debug)]
    struct MyStruct {
        a: i32,
        b: String,
        c: f64,
    }

    use builder::builder;
    #[test]
    fn test_vanilla() {
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
        let s = CustomBuilder::new()
            .a(10)
            .b(String::from("Hello"))
            .c(4.2)
            .build();
        assert_eq!(s.a, 10);
        assert_eq!(s.b, String::from("Hello"));
        assert_eq!(s.c, 4.2);
    }
}
