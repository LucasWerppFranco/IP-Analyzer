macro_rules! menu {
    ( $( $func:ident ),* ) => {
        {
            let items = vec![
                $(
                    MenuItem {
                        label: stringify!($func)
                            .replace("_", " ")
                            .to_uppercase(),
                        action: $func,
                    }
                ),*
            ];
            run_menu(items)
        }
    };
}

