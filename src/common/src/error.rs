#[macro_export]
macro_rules! SYNTAX_ERROR {
    ($l : expr) => {
        Result::Err(Error::new(ErrorKind::InvalidInput, $l))
    };
}

#[macro_export]
#[cfg(feature = "dbg")]
macro_rules! abc {
    ($i : expr) => {
        dbg!($i)
    };
}
#[macro_export]
#[cfg(not(feature = "dbg"))]
macro_rules! abc {
    ($i : expr) => {

    };
}

#[macro_export]
#[cfg(feature = "mul-dbg")]
macro_rules! mul_dbg {
    () => {};
    ( $($i : expr),* ) => {
        println!( $($i),*);
    }
}
#[macro_export]
#[cfg(not(feature = "mul-dbg"))]
macro_rules! mul_dbg {
    () => {};
    ( $($i : expr),* ) => {

    }
}

