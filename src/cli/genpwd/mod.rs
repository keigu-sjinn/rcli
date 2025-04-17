pub(crate) mod arg;
pub mod process;

macro_rules! def_pwd_length {
    ($val:expr) => {
        const DEFAULT_PWD_LENGTH: u8 = $val;
        const DEFAULT_PWD_LENGTH_STR: &str = stringify!($val);
    };
}

// 使用宏定义常量
def_pwd_length!(12);

const DEFAULT_PWD_UPPERCASE: &str = "true";
const DEFAULT_PWD_NUMBER: &str = "true";
const DEFAULT_PWD_SYMBOL: &str = "true";

const LOWER: &[u8] = b"abcdefghijklmnpqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";
