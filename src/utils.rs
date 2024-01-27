macro_rules! log {
    ( $( $t:tt )* ) => {
        eprintln!("sway-status [{}:{}] {}", file!(), line!(), format_args!( $( $t )* ));
    }
}
