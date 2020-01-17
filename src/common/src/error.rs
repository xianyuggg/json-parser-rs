#[macro_export]
macro_rules! SYNTAX_ERROR {
    ($l : expr) => {
        Result::Err(Error::new(ErrorKind::InvalidInput, $l))
    };
}
