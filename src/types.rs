use crate::DataResult;

pub trait Types {
    type List : ListType<Self>;
    type Map : MapType<Self>;
    type Object : ObjectType<Self>;
}

pub trait ObjectType<T: Types + ?Sized>: PartialEq + Clone {
    fn create_byte(value: i8) -> Self;
    fn create_short(value: i16) -> Self;
    fn create_int(value: i32) -> Self;
    fn create_long(value: i64) -> Self;
    fn create_float(value: f32) -> Self;
    fn create_double(value: f64) -> Self;
    fn create_byte_array(value: Vec<i8>) -> Self;
    fn create_short_array(value: Vec<i16>) -> Self;
    fn create_int_array(value: Vec<i32>) -> Self;
    fn create_long_array(value: Vec<i64>) -> Self;
    fn create_list(value: T::List) -> Self;
    fn create_map(value: T::Map) -> Self;
    fn create_string(value: String) -> Self;

    fn as_ref(&self) -> ObjectRef<T>;
    fn as_ref_mut(&mut self) -> ObjectRefMut<T>;
}

macro_rules! object_ref_impl {
    ($ref_type:ident, $copy_vec:expr) => {
        impl<'a, T: Types + ?Sized> $ref_type<'a, T> {
            pub fn is_number(&self) -> bool {
                matches!(self, Self::Byte(_) | Self::Short(_) | Self::Int(_) | Self::Long(_) | Self::Float(_) | Self::Double(_))
            }

            pub fn as_f64(&self) -> Option<f64> {
                Some(match self {
                    Self::Byte(v) => *v as f64,
                    Self::Short(v) => *v as f64,
                    Self::Int(v) => *v as f64,
                    Self::Long(v) => *v as f64,
                    Self::Float(v) => *v as f64,
                    Self::Double(v) => *v,
                    _ => return None
                })
            }

            pub fn as_i64(&self) -> Option<i64> {
                Some(match self {
                    Self::Byte(v) => *v as i64,
                    Self::Short(v) => *v as i64,
                    Self::Int(v) => *v as i64,
                    Self::Long(v) => *v as i64,
                    Self::Float(v) => *v as i64,
                    Self::Double(v) => *v as i64,
                    _ => return None
                })
            }

            pub fn clone_to_object(&self) -> T::Object {
                match self {
                    Self::Byte(v) => T::Object::create_byte(*v),
                    Self::Short(v) => T::Object::create_short(*v),
                    Self::Int(v) => T::Object::create_int(*v),
                    Self::Long(v) => T::Object::create_long(*v),
                    Self::Float(v) => T::Object::create_float(*v),
                    Self::Double(v) => T::Object::create_double(*v),
                    Self::ByteArray(v) => T::Object::create_byte_array($copy_vec(v)),
                    Self::ShortArray(v) => T::Object::create_short_array($copy_vec(v)),
                    Self::IntArray(v) => T::Object::create_int_array($copy_vec(v)),
                    Self::LongArray(v) => T::Object::create_long_array($copy_vec(v)),
                    Self::List(v) => T::Object::create_list((*v).clone()),
                    Self::Map(v) => T::Object::create_map((*v).clone()),
                    Self::String(v) => T::Object::create_string(v.to_string()),
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ObjectRef<'a, T: Types + ?Sized> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a [i8]),
    ShortArray(&'a [i16]),
    IntArray(&'a [i32]),
    LongArray(&'a [i64]),
    List(&'a T::List),
    Map(&'a T::Map),
    String(&'a str),
}
object_ref_impl!(ObjectRef, |v: &&[_]| Vec::from(*v));

#[derive(Debug, PartialEq)]
pub enum ObjectRefMut<'a, T: Types + ?Sized> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a mut Vec<i8>),
    ShortArray(&'a mut Vec<i16>),
    IntArray(&'a mut Vec<i32>),
    LongArray(&'a mut Vec<i64>),
    List(&'a mut T::List),
    Map(&'a mut T::Map),
    String(&'a mut str),
}
object_ref_impl!(ObjectRefMut, |v: &&mut Vec<_>| (*v).clone());

pub trait MapType<T: Types + ?Sized> : PartialEq + Clone + IntoIterator<Item=(String, T::Object)> {
    type KeyIter<'a> where Self: 'a;

    fn create_empty() -> Self;

    fn keys(&self) -> Self::KeyIter<'_>;

    fn has_key(&self, key: &str) -> bool;

    fn get(&self, key: &str) -> Option<&T::Object>;

    fn set(&mut self, key: String, value: T::Object);

    fn remove(&mut self, key: &str);

    fn clear(&mut self);

    fn size(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub trait ListType<T: Types + ?Sized> : PartialEq + Clone + IntoIterator<Item=T::Object> {
    fn create_empty() -> Self;

    fn get(&self, index: usize) -> &T::Object;

    fn set(&mut self, index: usize, value: T::Object);

    fn add(&mut self, value: T::Object);

    fn clear(&mut self);

    fn size(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

impl<T: Types + ?Sized> ListType<T> for Vec<T::Object> {
    #[inline]
    fn create_empty() -> Self {
        Vec::new()
    }

    #[inline]
    fn get(&self, index: usize) -> &T::Object {
        &self[index]
    }

    #[inline]
    fn set(&mut self, index: usize, value: T::Object) {
        self[index] = value;
    }

    #[inline]
    fn add(&mut self, value: T::Object) {
        self.push(value);
    }

    #[inline]
    fn clear(&mut self) {
        Vec::clear(self)
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T: Types + ?Sized, S: std::hash::BuildHasher + Clone + Default> MapType<T> for std::collections::HashMap<String, T::Object, S> {
    type KeyIter<'a> = impl IntoIterator<Item=&'a String> where Self: 'a;

    #[inline]
    fn create_empty() -> Self {
        Default::default()
    }

    #[inline]
    fn keys(&self) -> Self::KeyIter<'_> {
        std::collections::HashMap::keys(self)
    }

    #[inline]
    fn has_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &str) -> Option<&T::Object> {
        std::collections::HashMap::get(self, key)
    }

    #[inline]
    fn set(&mut self, key: String, value: T::Object) {
        self.insert(key, value);
    }

    #[inline]
    fn remove(&mut self, key: &str) {
        std::collections::HashMap::remove(self, key);
    }

    #[inline]
    fn clear(&mut self) {
        std::collections::HashMap::clear(self);
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "serde_json")]
pub struct SerdeJsonTypes;

#[cfg(feature = "serde_json")]
impl Types for SerdeJsonTypes {
    type List = Vec<serde_json::Value>;
    type Map = serde_json::Map<String, serde_json::Value>;
    type Object = serde_json::Value;
}

#[cfg(feature = "serde_json")]
impl MapType<SerdeJsonTypes> for serde_json::Map<String, serde_json::Value> {
    type KeyIter<'a> = impl IntoIterator<Item=&'a String> where T: 'a, S: 'a;

    #[inline]
    fn create_empty() -> Self {
        serde_json::Map::new()
    }

    #[inline]
    fn keys(&self) -> Self::KeyIter<'_> {
        serde_json::Map::keys(self)
    }

    #[inline]
    fn has_key(&self, key: &str) -> bool {
        serde_json::Map::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &str) -> Option<&serde_json::Value> {
        serde_json::Map::get(self, key)
    }

    #[inline]
    fn set(&mut self, key: String, value: serde_json::Value) {
        serde_json::Map::insert(self, key, value);
    }

    #[inline]
    fn remove(&mut self, key: &str) {
        serde_json::Map::remove(self, key);
    }

    #[inline]
    fn clear(&mut self) {
        serde_json::Map::clear(self);
    }

    #[inline]
    fn size(&self) -> usize {
        serde_json::Map::len(self)
    }
}

#[cfg(feature = "serde_json")]
impl ObjectType<SerdeJsonTypes> for serde_json::Value {
    #[inline]
    fn create_byte(value: i8) -> Self {
        Self::create_long(value as i64)
    }

    #[inline]
    fn create_short(value: i16) -> Self {
        Self::create_long(value as i64)
    }

    #[inline]
    fn create_int(value: i32) -> Self {
        Self::create_long(value as i64)
    }

    #[inline]
    fn create_long(value: i64) -> Self {
        serde_json::Value::Number(serde_json::Number::from(value))
    }

    #[inline]
    fn create_float(value: f32) -> Self {
        Self::create_double(value as f64)
    }

    #[inline]
    fn create_double(value: f64) -> Self {
        serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap())
    }

    fn create_byte_array(value: Vec<i8>) -> Self {
        serde_json::Value::Array(value.iter().map(Self::create_byte).collect())
    }

    fn create_short_array(value: Vec<i16>) -> Self {
        serde_json::Value::Array(value.iter().map(Self::create_short).collect())
    }

    fn create_int_array(value: Vec<i32>) -> Self {
        serde_json::Value::Array(value.iter().map(Self::create_int).collect())
    }

    fn create_long_array(value: Vec<i64>) -> Self {
        serde_json::Value::Array(value.iter().map(Self::create_long).collect())
    }

    #[inline]
    fn create_list(value: Vec<serde_json::Value>) -> Self {
        serde_json::Value::Array(value)
    }

    #[inline]
    fn create_map(value: serde_json::Map<String, serde_json::Value>) -> Self {
        serde_json::Value::Object(value)
    }

    #[inline]
    fn create_string(value: String) -> Self {
        serde_json::Value::String(value)
    }

    fn as_ref(&self) -> ObjectRef<SerdeJsonTypes> {
        match self {
            serde_json::Value::Number(n) => match n.as_f64() {
                Some(n) => ObjectRef::Double(n),
                None => ObjectRef::Long(n.as_i64().unwrap())
            }
            serde_json::Value::Array(arr) => ObjectRef::List(arr),
            serde_json::Value::Object(obj) => ObjectRef::Map(obj),
            serde_json::Value::String(str) => ObjectRef::String(str),
            serde_json::Value::Bool(b) => ObjectRef::Byte(if b {1} else {0}),
            serde_json::Value::Null => ObjectRef::Byte(0)
        }
    }

    fn as_ref_mut(&mut self) -> ObjectRefMut<SerdeJsonTypes> {
        match self {
            serde_json::Value::Number(n) => match n.as_f64() {
                Some(n) => ObjectRefMut::Double(n),
                None => ObjectRefMut::Long(n.as_i64().unwrap())
            }
            serde_json::Value::Array(arr) => ObjectRefMut::List(arr),
            serde_json::Value::Object(obj) => ObjectRefMut::Map(obj),
            serde_json::Value::String(str) => ObjectRefMut::String(str),
            serde_json::Value::Bool(b) => ObjectRefMut::Byte(if b {1} else {0}),
            serde_json::Value::Null => ObjectRefMut::Byte(0)
        }
    }
}

#[cfg(feature = "hematite-nbt")]
pub struct HematiteNbtTypes;

#[cfg(feature = "hematite-nbt")]
impl Types for HematiteNbtTypes {
    type List = Vec<nbt::Value>;
    type Map = std::collections::HashMap<String, nbt::Value>;
    type Object = nbt::Value;
}

#[cfg(feature = "hematite-nbt")]
impl ObjectType<HematiteNbtTypes> for nbt::Value {
    #[inline]
    fn create_byte(value: i8) -> Self {
        nbt::Value::Byte(value)
    }

    #[inline]
    fn create_short(value: i16) -> Self {
        nbt::Value::Short(value)
    }

    #[inline]
    fn create_int(value: i32) -> Self {
        nbt::Value::Int(value)
    }

    #[inline]
    fn create_long(value: i64) -> Self {
        nbt::Value::Long(value)
    }

    #[inline]
    fn create_float(value: f32) -> Self {
        nbt::Value::Float(value)
    }

    #[inline]
    fn create_double(value: f64) -> Self {
        nbt::Value::Double(value)
    }

    #[inline]
    fn create_byte_array(value: Vec<i8>) -> Self {
        nbt::Value::ByteArray(value)
    }

    fn create_short_array(value: Vec<i16>) -> Self {
        nbt::Value::IntArray(value.iter().map(|i| *i as i32).collect())
    }

    #[inline]
    fn create_int_array(value: Vec<i32>) -> Self {
        nbt::Value::IntArray(value)
    }

    #[inline]
    fn create_long_array(value: Vec<i64>) -> Self {
        nbt::Value::LongArray(value)
    }

    #[inline]
    fn create_list(value: Vec<nbt::Value>) -> Self {
        nbt::Value::List(value)
    }

    fn create_map(value: std::collections::HashMap<String, nbt::Value>) -> Self {
        nbt::Value::Compound(value)
    }

    fn create_string(value: String) -> Self {
        nbt::Value::String(value)
    }

    fn as_ref(&self) -> ObjectRef<HematiteNbtTypes> {
        match self {
            nbt::Value::Byte(b) => ObjectRef::Byte(*b),
            nbt::Value::Short(s) => ObjectRef::Short(*s),
            nbt::Value::Int(i) => ObjectRef::Int(*i),
            nbt::Value::Long(l) => ObjectRef::Long(*l),
            nbt::Value::Float(f) => ObjectRef::Float(*f),
            nbt::Value::Double(d) => ObjectRef::Double(*d),
            nbt::Value::ByteArray(arr) => ObjectRef::ByteArray(arr),
            nbt::Value::IntArray(arr) => ObjectRef::IntArray(arr),
            nbt::Value::LongArray(arr) => ObjectRef::LongArray(arr),
            nbt::Value::List(arr) => ObjectRef::List(arr),
            nbt::Value::Compound(obj) => ObjectRef::Map(obj),
            nbt::Value::String(str) => ObjectRef::String(str)
        }
    }

    fn as_ref_mut(&mut self) -> ObjectRefMut<HematiteNbtTypes> {
        match self {
            nbt::Value::Byte(b) => ObjectRefMut::Byte(*b),
            nbt::Value::Short(s) => ObjectRefMut::Short(*s),
            nbt::Value::Int(i) => ObjectRefMut::Int(*i),
            nbt::Value::Long(l) => ObjectRefMut::Long(*l),
            nbt::Value::Float(f) => ObjectRefMut::Float(*f),
            nbt::Value::Double(d) => ObjectRefMut::Double(*d),
            nbt::Value::ByteArray(arr) => ObjectRefMut::ByteArray(arr),
            nbt::Value::IntArray(arr) => ObjectRefMut::IntArray(arr),
            nbt::Value::LongArray(arr) => ObjectRefMut::LongArray(arr),
            nbt::Value::List(arr) => ObjectRefMut::List(arr),
            nbt::Value::Compound(obj) => ObjectRefMut::Map(obj),
            nbt::Value::String(str) => ObjectRefMut::String(str)
        }
    }
}
