use web_sys;

#[macro_use]
mod macros {
    #[macro_export]  macro_rules! log {
        ( $($t: tt)* ) => {
            web_sys::console::log_1(&format!($( $t )*).into());
        }
    }
}
