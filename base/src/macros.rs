#[macro_export]
macro_rules! ice {
    () => ({
        panic!("ICE: Please report an issue at https://github.com/bluespacengineers/revonslang/issues")
    });
    ($msg:expr) => ({
        panic!(concat!($msg, ". Please report an issue at https://github.com/bluespacengineers/revonslang/issues"))
    });
    ($fmt:expr, $($arg:tt)+) => ({
        panic!(concat!($fmt, ". Please report an issue at https://github.com/bluespacengineers/revonslang/issues"), $($arg)+)
    });
}
