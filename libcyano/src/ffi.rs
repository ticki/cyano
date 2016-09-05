pub fn undefined<T>() -> T {
    js!("return undefined");

    unreachable!();
}

pub fn null<T>() -> T {
    js!("return null");

    unreachable!();
}

#[macro_export]
macro_rules! raw_js {
    ($js:expr) => {
        concat!("[js?", $js, "?js]")
    };
}

#[macro_export]
macro_rules! js {
    ($js:expr) => {
        let _js_escape = raw_js!($js);
    };
}

#[macro_export]
macros_rules! export {
    (fn $name:ident($( $arg:ty ),*) -> $ret:ty { $body:block }) => {
        fn $name($( $arg ),*) -> $ret {
            // Export the function itself to the global namespace.
            js!(concat!("window.", stringify!($name), "=arguments.callee;");

            $body
        }
    };
    (pub fn $name:ident($( $arg:ty ),*) -> $ret:ty { $body:block }) => {
        pub fn $name($( $arg ),*) -> $ret {
            // Export the function itself to the global namespace.
            js!(concat!("window.", stringify!($name), "=arguments.callee;");

            $body
        }
    };
}

#[macro_export]
macro_rules! import {
    (fn $symb:ident() -> $ret:ty) => {
        fn $symb() -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "()"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty) -> $ret:ty) => {
        fn $symb($a0) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0)"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty, $a1:ty) -> $ret:ty) => {
        fn $symb($a0, $a1) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0,a1)"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty, $a1:ty, $a2:ty) -> $ret:ty) => {
        fn $symb($a0, $a1, $a2) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0,a1,a2)"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty, $a1:ty, $a2:ty, $a3:ty) -> $ret:ty) => {
        fn $symb($a0, $a1, $a2, $a3) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0,a1,a2,a3)"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty, $a1:ty, $a2:ty, $a3:ty, $a4:ty) -> $ret:ty) => {
        fn $symb($a0, $a1, $a2, $a3, $a4) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0,a1,a2,a3,a4)"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty, $a1:ty, $a2:ty, $a3:ty, $a4:ty, $a5:ty) -> $ret:ty) => {
        fn $symb($a0, $a1, $a2, $a3, $a4, $a5) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0,a1,a2,a3,a4,a5)"));

            unreachable!();
        }
    };
    (fn $symb:ident($a0:ty, $a1:ty, $a2:ty, $a3:ty, $a4:ty, $a5:ty, $a6:ty) -> $ret:ty) => {
        fn $symb($a0, $a1, $a2, $a3, $a4, $a5, $a6) -> $ret {
            let _escape = js!(concat!("return ", stringify!($symb), "(a0,a1,a2,a3,a4,a5,a6)"));

            unreachable!();
        }
    };
}
