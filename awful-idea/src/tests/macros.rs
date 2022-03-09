macro_rules! it_must {
    ($func_name:ident $code:block) => {
        #[test]
        fn $func_name() {
            assert!($code)
        }
    };

    ($($fn:ident $code:block)+) => {
        $(
            it_must! {
                $fn $code
            }
        )+
    }
}

pub(crate) use it_must;