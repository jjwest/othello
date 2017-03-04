#[macro_export]
macro_rules! err {
    ( $arg:expr ) => {{
        use std::io::Write;
        write!(&mut ::std::io::stderr(), "{}", $arg)
            .expect("Failed to write to stderr");
        
    }};
    ( $fmt:expr, $( $arg:tt )* ) => {{
        use std::io::Write;
        write!(&mut ::std::io::stderr(), $fmt, $( $arg )*)
            .expect("Failed to write to stderr");
    }}
}

#[macro_export]
macro_rules! errln {
    ( $arg:expr ) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), "{}", $arg)
            .expect("Failed to write to stderr");
        
    }};
    ( $fmt:expr, $( $arg:tt )* ) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $fmt, $( $arg )*)
            .expect("Failed to write to stderr");
    }}
}
