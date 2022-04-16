use std::borrow::Borrow;
use std::hash::Hash;
use std::slice;

pub trait Types: 'static {
    type List : ListType<Self>;
    type Map : MapType<Self>;
    type Object : ObjectType<Self>;
}

pub trait ObjectType<T: 'static + Types + ?Sized>: PartialEq + Clone {
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

    fn as_i64(&self) -> Option<i64> {
        self.as_ref().as_i64()
    }

    fn as_f64(&self) -> Option<f64> {
        self.as_ref().as_f64()
    }

    fn as_string(&self) -> Option<&str> {
        self.as_ref().into_string_ref()
    }

    fn as_string_mut(&mut self) -> Option<&mut str> {
        self.as_ref_mut().into_string_ref()
    }

    fn as_list(&self) -> Option<&T::List> {
        self.as_ref().into_list_ref()
    }

    fn as_list_mut(&mut self) -> Option<&mut T::List> {
        self.as_ref_mut().into_list_ref()
    }

    fn as_map(&self) -> Option<&T::Map> {
        self.as_ref().into_map_ref()
    }

    fn as_map_mut(&mut self) -> Option<&mut T::Map> {
        self.as_ref_mut().into_map_ref()
    }
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

impl<'a, T: Types + ?Sized> ObjectRef<'a, T> {
    pub fn into_string_ref(self) -> Option<&'a str> {
        match self {
            Self::String(str) => Some(str),
            _ => None
        }
    }

    pub fn into_list_ref(self) -> Option<&'a T::List> {
        match self {
            Self::List(arr) => Some(arr),
            _ => None
        }
    }

    pub fn into_map_ref(self) -> Option<&'a T::Map> {
        match self {
            Self::Map(obj) => Some(obj),
            _ => None
        }
    }
}

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

impl<'a, T: Types + ?Sized> ObjectRefMut<'a, T> {
    pub fn into_string_ref(self) -> Option<&'a mut str> {
        match self {
            Self::String(str) => Some(str),
            _ => None
        }
    }

    pub fn into_list_ref(self) -> Option<&'a mut T::List> {
        match self {
            Self::List(arr) => Some(arr),
            _ => None
        }
    }

    pub fn into_map_ref(self) -> Option<&'a mut T::Map> {
        match self {
            Self::Map(obj) => Some(obj),
            _ => None
        }
    }
}

pub trait MapType<T: Types + ?Sized> : PartialEq + Clone + IntoIterator<Item=(String, T::Object)> {
    type KeyIter<'a> : Iterator<Item = &'a String> where Self: 'a;
    type ValueIter<'a> : Iterator<Item = &'a T::Object> where Self: 'a;
    type ValueIterMut<'a> : Iterator<Item = &'a mut T::Object> where Self: 'a;

    fn create_empty() -> Self;

    fn keys(&self) -> Self::KeyIter<'_>;

    fn has_key<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> bool where String: Borrow<Q>;

    fn values(&self) -> Self::ValueIter<'_>;

    fn values_mut(&mut self) -> Self::ValueIterMut<'_>;

    fn get<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<&T::Object> where String: Borrow<Q>;

    fn get_mut<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<&mut T::Object> where String: Borrow<Q>;

    fn set(&mut self, key: impl Into<String>, value: T::Object);

    fn remove<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<T::Object> where String: Borrow<Q>;

    fn clear(&mut self);

    fn size(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn get_i64<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<i64> where String: Borrow<Q> {
        self.get(key).and_then(T::Object::as_i64)
    }

    fn get_f64<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<f64> where String: Borrow<Q> {
        self.get(key).and_then(T::Object::as_f64)
    }

    fn get_string<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<&str> where String: Borrow<Q> {
        self.get(key).and_then(T::Object::as_string)
    }

    fn get_string_mut<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<&mut str> where String: Borrow<Q> {
        self.get_mut(key).and_then(T::Object::as_string_mut)
    }

    fn get_list<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<&T::List> where String: Borrow<Q> {
        self.get(key).and_then(T::Object::as_list)
    }

    fn get_list_mut<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<&mut T::List> where String: Borrow<Q> {
        self.get_mut(key).and_then(T::Object::as_list_mut)
    }

    fn get_map<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<&T::Map> where String: Borrow<Q> {
        self.get(key).and_then(T::Object::as_map)
    }

    fn get_map_mut<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<&mut T::Map> where String: Borrow<Q> {
        self.get_mut(key).and_then(T::Object::as_map_mut)
    }

    fn rename_key<Q: ?Sized + Hash + Eq + Ord>(&mut self, from: &Q, to: impl Into<String>) where String: Borrow<Q> {
        if let Some(value) = self.remove(from) {
            self.set(to, value);
        }
    }
}

pub trait ListType<T: Types + ?Sized> : PartialEq + Clone {
    type Iter<'a> : Iterator<Item = &'a T::Object> where Self: 'a;
    type IterMut<'a> : Iterator<Item = &'a mut T::Object> where Self: 'a;

    fn create_empty() -> Self;

    fn get(&self, index: usize) -> &T::Object;

    fn set(&mut self, index: usize, value: T::Object);

    fn add(&mut self, value: T::Object);

    fn clear(&mut self);

    fn size(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

impl<T: Types + ?Sized> ListType<T> for Vec<T::Object> {
    type Iter<'a> = impl Iterator<Item = &'a T::Object>;
    type IterMut<'a> = impl Iterator<Item = &'a mut T::Object>;

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

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        <&Vec<T::Object>>::into_iter(self)
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut Vec<T::Object>>::into_iter(self)
    }
}

impl<T: Types + ?Sized, S: std::hash::BuildHasher + Clone + Default> MapType<T> for std::collections::HashMap<String, T::Object, S> {
    type KeyIter<'a> = impl Iterator<Item=&'a String> where S: 'a;
    type ValueIter<'a> = impl Iterator<Item=&'a T::Object> where S: 'a;
    type ValueIterMut<'a> = impl Iterator<Item=&'a mut T::Object> where S: 'a;

    #[inline]
    fn create_empty() -> Self {
        Default::default()
    }

    #[inline]
    fn keys(&self) -> Self::KeyIter<'_> {
        std::collections::HashMap::keys(self)
    }

    #[inline]
    fn has_key<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> bool where String: Borrow<Q> {
        self.contains_key(key)
    }

    #[inline]
    fn values(&self) -> Self::ValueIter<'_> {
        std::collections::HashMap::values(self)
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValueIterMut<'_> {
        std::collections::HashMap::values_mut(self)
    }

    #[inline]
    fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&T::Object> where String: Borrow<Q> {
        std::collections::HashMap::get(self, key)
    }

    #[inline]
    fn get_mut<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<&mut T::Object> where String: Borrow<Q> {
        std::collections::HashMap::get_mut(self, key)
    }

    #[inline]
    fn set(&mut self, key: impl Into<String>, value: T::Object) {
        self.insert(key.into(), value);
    }

    #[inline]
    fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<T::Object> where String: Borrow<Q> {
        std::collections::HashMap::remove(self, key)
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

#[cfg(feature = "indexmap")]
impl<T: Types + ?Sized, S: std::hash::BuildHasher + Clone + Default> MapType<T> for indexmap::IndexMap<String, T::Object, S> {
    type KeyIter<'a> = impl Iterator<Item=&'a String> where S: 'a;
    type ValueIter<'a> = impl Iterator<Item=&'a T::Object> where S: 'a;
    type ValueIterMut<'a> = impl Iterator<Item=&'a mut T::Object> where S: 'a;

    #[inline]
    fn create_empty() -> Self {
        Default::default()
    }

    #[inline]
    fn keys(&self) -> Self::KeyIter<'_> {
        indexmap::IndexMap::keys(self)
    }

    #[inline]
    fn has_key<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> bool where String: Borrow<Q> {
        indexmap::IndexMap::contains_key(self, key)
    }

    #[inline]
    fn values(&self) -> Self::ValueIter<'_> {
        indexmap::IndexMap::values(self)
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValueIterMut<'_> {
        indexmap::IndexMap::values_mut(self)
    }

    #[inline]
    fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&T::Object> where String: Borrow<Q> {
        indexmap::IndexMap::get(self, key)
    }

    #[inline]
    fn get_mut<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<&mut T::Object> where String: Borrow<Q> {
        indexmap::IndexMap::get_mut(self, key)
    }

    #[inline]
    fn set(&mut self, key: impl Into<String>, value: T::Object) {
        indexmap::IndexMap::insert(self, key.into(), value);
    }

    #[inline]
    fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<T::Object> where String: Borrow<Q> {
        indexmap::IndexMap::remove(self, key)
    }

    #[inline]
    fn clear(&mut self) {
        indexmap::IndexMap::clear(self);
    }

    #[inline]
    fn size(&self) -> usize {
        indexmap::IndexMap::len(self)
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
    type KeyIter<'a> = impl Iterator<Item=&'a String>;
    type ValueIter<'a> = impl Iterator<Item=&'a serde_json::Value>;
    type ValueIterMut<'a> = impl Iterator<Item=&'a mut serde_json::Value>;

    #[inline]
    fn create_empty() -> Self {
        serde_json::Map::new()
    }

    #[inline]
    fn keys(&self) -> Self::KeyIter<'_> {
        serde_json::Map::keys(self)
    }

    #[inline]
    fn has_key<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> bool where String: Borrow<Q> {
        serde_json::Map::contains_key(self, key)
    }

    #[inline]
    fn values(&self) -> Self::ValueIter<'_> {
        serde_json::Map::values(self)
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValueIterMut<'_> {
        serde_json::Map::values_mut(self)
    }

    #[inline]
    fn get<Q: ?Sized + Hash + Eq + Ord>(&self, key: &Q) -> Option<&serde_json::Value> where String: Borrow<Q> {
        serde_json::Map::get(self, key)
    }

    #[inline]
    fn get_mut<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<&mut serde_json::Value> where String: Borrow<Q> {
        serde_json::Map::get_mut(self, key)
    }

    #[inline]
    fn set(&mut self, key: impl Into<String>, value: serde_json::Value) {
        serde_json::Map::insert(self, key.into(), value);
    }

    #[inline]
    fn remove<Q: ?Sized + Hash + Eq + Ord>(&mut self, key: &Q) -> Option<serde_json::Value> where String: Borrow<Q> {
        serde_json::Map::remove(self, key)
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
        serde_json::Value::Array(value.into_iter().map(Self::create_byte).collect())
    }

    fn create_short_array(value: Vec<i16>) -> Self {
        serde_json::Value::Array(value.into_iter().map(Self::create_short).collect())
    }

    fn create_int_array(value: Vec<i32>) -> Self {
        serde_json::Value::Array(value.into_iter().map(Self::create_int).collect())
    }

    fn create_long_array(value: Vec<i64>) -> Self {
        serde_json::Value::Array(value.into_iter().map(Self::create_long).collect())
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
            serde_json::Value::Bool(b) => ObjectRef::Byte(if *b {1} else {0}),
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
            serde_json::Value::Bool(b) => ObjectRefMut::Byte(if *b {1} else {0}),
            serde_json::Value::Null => ObjectRefMut::Byte(0)
        }
    }
}

#[cfg(feature = "hematite-nbt")]
pub struct HematiteNbtTypes;

#[cfg(feature = "hematite-nbt")]
impl Types for HematiteNbtTypes {
    type List = Vec<nbt::Value>;
    type Map = nbt::Map<String, nbt::Value>;
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
        nbt::Value::IntArray(value.into_iter().map(|i| i as i32).collect())
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

#[cfg(feature = "quartz_nbt")]
pub struct QuartzNbtTypes;

#[cfg(feature = "quartz_nbt")]
impl Types for QuartzNbtTypes {
    type List = quartz_nbt::NbtList;
    type Map = quartz_nbt::NbtCompound;
    type Object = quartz_nbt::NbtTag;
}

#[cfg(feature = "quartz_nbt")]
impl ListType<QuartzNbtTypes> for quartz_nbt::NbtList {
    type Iter<'a> = impl Iterator<Item = &'a quartz_nbt::NbtTag>;
    type IterMut<'a> = impl Iterator<Item = &'a mut quartz_nbt::NbtTag>;

    #[inline]
    fn create_empty() -> Self {
        quartz_nbt::NbtList::new()
    }

    fn get(&self, index: usize) -> &quartz_nbt::NbtTag {
        quartz_nbt::NbtList::get(self, index).expect("Index out of bounds")
    }

    #[inline]
    fn set(&mut self, index: usize, value: quartz_nbt::NbtTag) {
        self.inner_mut()[index] = value;
    }

    #[inline]
    fn add(&mut self, value: quartz_nbt::NbtTag) {
        quartz_nbt::NbtList::push(self, value)
    }

    #[inline]
    fn clear(&mut self) {
        self.inner_mut().clear()
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        <&quartz_nbt::NbtList>::into_iter(self)
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut quartz_nbt::NbtList>::into_iter(self)
    }
}

#[cfg(feature = "quartz_nbt")]
impl MapType<QuartzNbtTypes> for quartz_nbt::NbtCompound {
    type KeyIter<'a> = impl Iterator<Item = &'a String>;
    type ValueIter<'a> = impl Iterator<Item = &'a quartz_nbt::NbtTag>;
    type ValueIterMut<'a> = impl Iterator<Item = &'a mut quartz_nbt::NbtTag>;

    #[inline]
    fn create_empty() -> Self {
        quartz_nbt::NbtCompound::new()
    }

    #[inline]
    fn keys(&self) -> Self::KeyIter<'_> {
        self.inner().keys()
    }

    #[inline]
    fn has_key<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> bool where String: Borrow<Q> {
        self.contains_key(key)
    }

    #[inline]
    fn values(&self) -> Self::ValueIter<'_> {
        self.inner().values()
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValueIterMut<'_> {
        self.inner_mut().values_mut()
    }

    fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&quartz_nbt::NbtTag> where String: Borrow<Q> {
        self.inner().get(key)
    }

    fn get_mut<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<&mut quartz_nbt::NbtTag> where String: Borrow<Q> {
        self.inner_mut().get_mut(key)
    }

    #[inline]
    fn set(&mut self, key: impl Into<String>, value: quartz_nbt::NbtTag) {
        quartz_nbt::NbtCompound::insert(self, key.into(), value);
    }

    #[inline]
    fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<quartz_nbt::NbtTag> where String: Borrow<Q> {
        self.inner_mut().remove(key)
    }

    #[inline]
    fn clear(&mut self) {
        self.inner_mut().clear()
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "quartz_nbt")]
impl ObjectType<QuartzNbtTypes> for quartz_nbt::NbtTag {
    #[inline]
    fn create_byte(value: i8) -> Self {
        quartz_nbt::NbtTag::Byte(value)
    }

    #[inline]
    fn create_short(value: i16) -> Self {
        quartz_nbt::NbtTag::Short(value)
    }

    #[inline]
    fn create_int(value: i32) -> Self {
        quartz_nbt::NbtTag::Int(value)
    }

    #[inline]
    fn create_long(value: i64) -> Self {
        quartz_nbt::NbtTag::Long(value)
    }

    #[inline]
    fn create_float(value: f32) -> Self {
        quartz_nbt::NbtTag::Float(value)
    }

    #[inline]
    fn create_double(value: f64) -> Self {
        quartz_nbt::NbtTag::Double(value)
    }

    #[inline]
    fn create_byte_array(value: Vec<i8>) -> Self {
        quartz_nbt::NbtTag::ByteArray(value)
    }

    fn create_short_array(value: Vec<i16>) -> Self {
        Self::create_int_array(value.into_iter().map(|s| s as i32).collect())
    }

    #[inline]
    fn create_int_array(value: Vec<i32>) -> Self {
        quartz_nbt::NbtTag::IntArray(value)
    }

    #[inline]
    fn create_long_array(value: Vec<i64>) -> Self {
        quartz_nbt::NbtTag::LongArray(value)
    }

    #[inline]
    fn create_list(value: quartz_nbt::NbtList) -> Self {
        quartz_nbt::NbtTag::List(value)
    }

    #[inline]
    fn create_map(value: quartz_nbt::NbtCompound) -> Self {
        quartz_nbt::NbtTag::Compound(value)
    }

    #[inline]
    fn create_string(value: String) -> Self {
        quartz_nbt::NbtTag::String(value)
    }

    fn as_ref(&self) -> ObjectRef<QuartzNbtTypes> {
        match self {
            quartz_nbt::NbtTag::Byte(v) => ObjectRef::Byte(*v),
            quartz_nbt::NbtTag::Short(v) => ObjectRef::Short(*v),
            quartz_nbt::NbtTag::Int(v) => ObjectRef::Int(*v),
            quartz_nbt::NbtTag::Long(v) => ObjectRef::Long(*v),
            quartz_nbt::NbtTag::Float(v) => ObjectRef::Float(*v),
            quartz_nbt::NbtTag::Double(v) => ObjectRef::Double(*v),
            quartz_nbt::NbtTag::ByteArray(v) => ObjectRef::ByteArray(v),
            quartz_nbt::NbtTag::IntArray(v) => ObjectRef::IntArray(v),
            quartz_nbt::NbtTag::LongArray(v) => ObjectRef::LongArray(v),
            quartz_nbt::NbtTag::List(v) => ObjectRef::List(v),
            quartz_nbt::NbtTag::Compound(v) => ObjectRef::Map(v),
            quartz_nbt::NbtTag::String(v) => ObjectRef::String(v),
        }
    }

    fn as_ref_mut(&mut self) -> ObjectRefMut<QuartzNbtTypes> {
        match self {
            quartz_nbt::NbtTag::Byte(v) => ObjectRefMut::Byte(*v),
            quartz_nbt::NbtTag::Short(v) => ObjectRefMut::Short(*v),
            quartz_nbt::NbtTag::Int(v) => ObjectRefMut::Int(*v),
            quartz_nbt::NbtTag::Long(v) => ObjectRefMut::Long(*v),
            quartz_nbt::NbtTag::Float(v) => ObjectRefMut::Float(*v),
            quartz_nbt::NbtTag::Double(v) => ObjectRefMut::Double(*v),
            quartz_nbt::NbtTag::ByteArray(v) => ObjectRefMut::ByteArray(v),
            quartz_nbt::NbtTag::IntArray(v) => ObjectRefMut::IntArray(v),
            quartz_nbt::NbtTag::LongArray(v) => ObjectRefMut::LongArray(v),
            quartz_nbt::NbtTag::List(v) => ObjectRefMut::List(v),
            quartz_nbt::NbtTag::Compound(v) => ObjectRefMut::Map(v),
            quartz_nbt::NbtTag::String(v) => ObjectRefMut::String(v),
        }
    }
}
