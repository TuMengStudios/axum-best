/// 支持所有基础类型的枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    // 有符号整数类型
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),

    // 无符号整数类型
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    // 浮点类型
    F32(f32),
    F64(f64),

    // 布尔类型
    Bool(bool),

    // 字符类型
    Char(char),

    // 字符串类型
    Str(String),

    // 空值
    Null,
}

#[tokio::test]
async fn todo2() {
    let _p = Primitive::from(23);
    println!("{} {} --- {:?}", _p.to_string(), _p.type_name(), _p)
}
impl Primitive {
    /// 获取值的类型名称
    pub fn type_name(&self) -> &'static str {
        match self {
            Primitive::I8(_) => "i8",
            Primitive::I16(_) => "i16",
            Primitive::I32(_) => "i32",
            Primitive::I64(_) => "i64",
            Primitive::I128(_) => "i128",
            Primitive::Isize(_) => "isize",
            Primitive::U8(_) => "u8",
            Primitive::U16(_) => "u16",
            Primitive::U32(_) => "u32",
            Primitive::U64(_) => "u64",
            Primitive::U128(_) => "u128",
            Primitive::Usize(_) => "usize",
            Primitive::F32(_) => "f32",
            Primitive::F64(_) => "f64",
            Primitive::Bool(_) => "bool",
            Primitive::Char(_) => "char",
            Primitive::Str(_) => "String",
            Primitive::Null => "null",
        }
    }

    /// 转换为字符串表示
    pub fn to_string(&self) -> String {
        match self {
            Primitive::I8(v) => v.to_string(),
            Primitive::I16(v) => v.to_string(),
            Primitive::I32(v) => v.to_string(),
            Primitive::I64(v) => v.to_string(),
            Primitive::I128(v) => v.to_string(),
            Primitive::Isize(v) => v.to_string(),
            Primitive::U8(v) => v.to_string(),
            Primitive::U16(v) => v.to_string(),
            Primitive::U32(v) => v.to_string(),
            Primitive::U64(v) => v.to_string(),
            Primitive::U128(v) => v.to_string(),
            Primitive::Usize(v) => v.to_string(),
            Primitive::F32(v) => v.to_string(),
            Primitive::F64(v) => v.to_string(),
            Primitive::Bool(v) => v.to_string(),
            Primitive::Char(v) => v.to_string(),
            Primitive::Str(v) => v.clone(),
            Primitive::Null => "null".to_string(),
        }
    }

    /// 检查是否为数值类型
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Primitive::I8(_)
                | Primitive::I16(_)
                | Primitive::I32(_)
                | Primitive::I64(_)
                | Primitive::I128(_)
                | Primitive::Isize(_)
                | Primitive::U8(_)
                | Primitive::U16(_)
                | Primitive::U32(_)
                | Primitive::U64(_)
                | Primitive::U128(_)
                | Primitive::Usize(_)
                | Primitive::F32(_)
                | Primitive::F64(_)
        )
    }

    /// 检查是否为整数类型
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Primitive::I8(_)
                | Primitive::I16(_)
                | Primitive::I32(_)
                | Primitive::I64(_)
                | Primitive::I128(_)
                | Primitive::Isize(_)
                | Primitive::U8(_)
                | Primitive::U16(_)
                | Primitive::U32(_)
                | Primitive::U64(_)
                | Primitive::U128(_)
                | Primitive::Usize(_)
        )
    }

    /// 检查是否为浮点类型
    pub fn is_float(&self) -> bool {
        matches!(self, Primitive::F32(_) | Primitive::F64(_))
    }
}

// 为常见类型实现 From trait
impl From<i32> for Primitive {
    fn from(value: i32) -> Self {
        Primitive::I32(value)
    }
}

impl From<i64> for Primitive {
    fn from(value: i64) -> Self {
        Primitive::I64(value)
    }
}

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Primitive::Bool(value)
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        Primitive::Str(value)
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Self {
        Primitive::Str(value.to_string())
    }
}

impl From<f32> for Primitive {
    fn from(value: f32) -> Self {
        Primitive::F32(value)
    }
}

impl From<f64> for Primitive {
    fn from(value: f64) -> Self {
        Primitive::F64(value)
    }
}

impl From<char> for Primitive {
    fn from(value: char) -> Self {
        Primitive::Char(value)
    }
}

// 为其他整数类型实现 From trait
impl From<i8> for Primitive {
    fn from(value: i8) -> Self {
        Primitive::I8(value)
    }
}

impl From<i16> for Primitive {
    fn from(value: i16) -> Self {
        Primitive::I16(value)
    }
}

impl From<i128> for Primitive {
    fn from(value: i128) -> Self {
        Primitive::I128(value)
    }
}

impl From<isize> for Primitive {
    fn from(value: isize) -> Self {
        Primitive::Isize(value)
    }
}

impl From<u8> for Primitive {
    fn from(value: u8) -> Self {
        Primitive::U8(value)
    }
}

impl From<u16> for Primitive {
    fn from(value: u16) -> Self {
        Primitive::U16(value)
    }
}

impl From<u32> for Primitive {
    fn from(value: u32) -> Self {
        Primitive::U32(value)
    }
}

impl From<u64> for Primitive {
    fn from(value: u64) -> Self {
        Primitive::U64(value)
    }
}

impl From<u128> for Primitive {
    fn from(value: u128) -> Self {
        Primitive::U128(value)
    }
}

impl From<usize> for Primitive {
    fn from(value: usize) -> Self {
        Primitive::Usize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_creation() {
        let p1 = Primitive::I32(42);
        let p2 = Primitive::from(42i32);
        let p3 = Primitive::Bool(true);
        let p4 = Primitive::from("hello");
        let p5 = Primitive::F64(3.14);

        assert_eq!(p1, p2);
        assert_eq!(p3.to_string(), "true");
        assert_eq!(p4.to_string(), "hello");
        assert_eq!(p5.type_name(), "f64");
    }

    #[test]
    fn test_type_checks() {
        let int_val = Primitive::I32(42);
        let float_val = Primitive::F32(3.14);
        let bool_val = Primitive::Bool(true);
        let str_val = Primitive::Str("test".to_string());

        assert!(int_val.is_numeric());
        assert!(int_val.is_integer());
        assert!(!int_val.is_float());

        assert!(float_val.is_numeric());
        assert!(!float_val.is_integer());
        assert!(float_val.is_float());

        assert!(!bool_val.is_numeric());
        assert!(!str_val.is_numeric());
    }
}
