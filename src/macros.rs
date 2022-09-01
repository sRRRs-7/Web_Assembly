use web_sys;

#[macro_use]
mod macros {
    #[macro_export]  macro_rules! log {
        ( $($t: tt)* ) => {
            web_sys::console::log_1(&format!($( $t )*).into());
        }
    }
    #[macro_export] macro_rules! round {
        ($x:expr, $scale:expr) => (($x * $scale).round() / $scale)
    }
    #[macro_export] macro_rules! ceil {
        ($x:expr, $scale:expr) => (($x * $scale).ceil() / $scale)
    }
    #[macro_export] macro_rules! floor {
        ($x:expr, $scale:expr) => (($x * $scale).floor() / $scale)
    }

}
