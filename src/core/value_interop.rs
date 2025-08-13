use crate::core::Value;
use anyhow::anyhow;

/// Trait for consuming a Value and converting it to a Rust type.
/// Use this for non-reference types like i64, f64, bool, String, etc.
#[allow(dead_code)]
pub trait FromValue<T> {
    fn from_value(value: Value) -> Result<T, anyhow::Error>;
}

/// Trait for borrowing a Value as a reference to a Rust type.
/// Use this for borrowing internal data like `&Vec<Value>` from Value::List or `&Point` from PointRef.
/// Returns a guard that derefs to the target type.
#[allow(dead_code)]
pub trait BorrowValueAs<T: ?Sized> {
    type Guard<'a>: std::ops::Deref<Target = T>
    where
        Self: 'a;
    fn borrow_value_as(&self) -> Result<Self::Guard<'_>, anyhow::Error>;
}

/// Trait for mutably borrowing a Value as a reference to a Rust type.
/// Use this for mutably borrowing internal data like `&mut Vec<Value>` from Value::List or `&mut Point` from PointRef.
/// Returns a guard that derefs to the target type.
#[allow(dead_code)]
pub trait BorrowMutValueAs<T: ?Sized> {
    type Guard<'a>: std::ops::DerefMut<Target = T>
    where
        Self: 'a;
    fn borrow_mut_value_as(&mut self) -> Result<Self::Guard<'_>, anyhow::Error>;
}

/// Trait for converting a Rust type to a Value.
/// Use this for Rust type → Value conversion.
#[allow(dead_code)]
pub trait ToValue {
    fn to_value(self) -> Value;
}

// FromValue implementations for basic types

impl FromValue<f64> for f64 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as f64),
            _ => Err(anyhow!("Cannot convert {} to f64", value.type_name())),
        }
    }
}

impl FromValue<i64> for i64 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(anyhow!("Cannot convert {} to i64", value.type_name())),
        }
    }
}

impl FromValue<bool> for bool {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(anyhow!("Cannot convert {} to bool", value.type_name())),
        }
    }
}

impl FromValue<String> for String {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(anyhow!("Cannot convert {} to String", value.type_name())),
        }
    }
}

impl FromValue<()> for () {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Unit => Ok(()),
            _ => Err(anyhow!("Cannot convert {} to unit type", value.type_name())),
        }
    }
}

// FromValue implementations for additional integer types with range checking

impl FromValue<i32> for i32 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                i32::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for i32"))
            }
            _ => Err(anyhow!("Cannot convert {} to i32", value.type_name())),
        }
    }
}

impl FromValue<i16> for i16 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                i16::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for i16"))
            }
            _ => Err(anyhow!("Cannot convert {} to i16", value.type_name())),
        }
    }
}

impl FromValue<i8> for i8 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                i8::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for i8"))
            }
            _ => Err(anyhow!("Cannot convert {} to i8", value.type_name())),
        }
    }
}

impl FromValue<u64> for u64 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                u64::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for u64"))
            }
            _ => Err(anyhow!("Cannot convert {} to u64", value.type_name())),
        }
    }
}

impl FromValue<u32> for u32 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                u32::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for u32"))
            }
            _ => Err(anyhow!("Cannot convert {} to u32", value.type_name())),
        }
    }
}

impl FromValue<u16> for u16 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                u16::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for u16"))
            }
            _ => Err(anyhow!("Cannot convert {} to u16", value.type_name())),
        }
    }
}

impl FromValue<u8> for u8 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                u8::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for u8"))
            }
            _ => Err(anyhow!("Cannot convert {} to u8", value.type_name())),
        }
    }
}

impl FromValue<usize> for usize {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                usize::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for usize"))
            }
            _ => Err(anyhow!("Cannot convert {} to usize", value.type_name())),
        }
    }
}

impl FromValue<isize> for isize {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Int(i) => {
                isize::try_from(i).map_err(|_| anyhow!("i64 value {i} is out of range for isize"))
            }
            _ => Err(anyhow!("Cannot convert {} to isize", value.type_name())),
        }
    }
}

impl FromValue<f32> for f32 {
    fn from_value(value: Value) -> Result<Self, anyhow::Error> {
        match value {
            Value::Float(f) => Ok(f as f32),
            Value::Int(i) => Ok(i as f32),
            _ => Err(anyhow!("Cannot convert {} to f32", value.type_name())),
        }
    }
}

// ToValue implementations for Rust types → Value conversion

impl ToValue for f64 {
    fn to_value(self) -> Value {
        Value::Float(self)
    }
}

impl ToValue for i64 {
    fn to_value(self) -> Value {
        Value::Int(self)
    }
}

impl ToValue for bool {
    fn to_value(self) -> Value {
        Value::Bool(self)
    }
}

impl ToValue for String {
    fn to_value(self) -> Value {
        Value::String(self)
    }
}

impl ToValue for &str {
    fn to_value(self) -> Value {
        Value::String(self.to_string())
    }
}

impl ToValue for () {
    fn to_value(self) -> Value {
        Value::Unit
    }
}

impl ToValue for i32 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for i16 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for i8 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for u64 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for u32 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for u16 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for u8 {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for usize {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for isize {
    fn to_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl ToValue for f32 {
    fn to_value(self) -> Value {
        Value::Float(self as f64)
    }
}

// BorrowValueAs implementations on Value for built-in container types

impl BorrowValueAs<Vec<Value>> for Value {
    type Guard<'a> = std::cell::Ref<'a, Vec<Value>>;

    fn borrow_value_as(&self) -> Result<Self::Guard<'_>, anyhow::Error> {
        match self {
            Value::List(list_ref) => Ok(list_ref.borrow()),
            _ => Err(anyhow!("Cannot borrow {} as Vec<Value>", self.type_name())),
        }
    }
}

impl BorrowMutValueAs<Vec<Value>> for Value {
    type Guard<'a> = std::cell::RefMut<'a, Vec<Value>>;

    fn borrow_mut_value_as(&mut self) -> Result<Self::Guard<'_>, anyhow::Error> {
        match self {
            Value::List(list_ref) => Ok(list_ref.borrow_mut()),
            _ => Err(anyhow!(
                "Cannot borrow {} as mut Vec<Value>",
                self.type_name()
            )),
        }
    }
}

impl BorrowValueAs<indexmap::IndexMap<String, Value>> for Value {
    type Guard<'a> = std::cell::Ref<'a, indexmap::IndexMap<String, Value>>;

    fn borrow_value_as(&self) -> Result<Self::Guard<'_>, anyhow::Error> {
        match self {
            Value::Dict(dict_ref) => Ok(dict_ref.borrow()),
            _ => Err(anyhow!(
                "Cannot borrow {} as IndexMap<String, Value>",
                self.type_name()
            )),
        }
    }
}

impl BorrowMutValueAs<indexmap::IndexMap<String, Value>> for Value {
    type Guard<'a> = std::cell::RefMut<'a, indexmap::IndexMap<String, Value>>;

    fn borrow_mut_value_as(&mut self) -> Result<Self::Guard<'_>, anyhow::Error> {
        match self {
            Value::Dict(dict_ref) => Ok(dict_ref.borrow_mut()),
            _ => Err(anyhow!(
                "Cannot borrow {} as mut IndexMap<String, Value>",
                self.type_name()
            )),
        }
    }
}

// BorrowValueAs implementations for user-defined types will be generated by the proc macro
// Example: impl BorrowValueAs<Point> for Value { ... }
