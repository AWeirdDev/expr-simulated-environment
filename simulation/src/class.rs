#[macro_export]
macro_rules! class {
    (
        struct $name:ident {
            $(
                fn $fnname:ident($($args:ident: $argty:ty),*) $body:block
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;
    };
}

class!(
    struct Class {
        fn hello() {
            println!("hello");
        }
    }
);
