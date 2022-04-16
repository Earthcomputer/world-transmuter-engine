#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod convert;
mod types;
mod utils;

#[cfg(feature = "ahash")]
type RandomState = ahash::RandomState;
#[cfg(not(feature = "ahash"))]
type RandomState = std::collections::hash_map::RandomState;
#[cfg(feature = "indexmap")]
pub type Map<K, V> = indexmap::IndexMap<K, V, RandomState>;
#[cfg(not(feature = "indexmap"))]
pub type Map<K, V> = std::collections::HashMap<K, V, RandomState>;

pub use crate::convert::*;
pub use crate::types::*;
pub use crate::utils::*;

#[cfg(test)]
#[cfg(feature = "quartz_nbt")]
mod tests {
    type TestTypes = QuartzNbtTypes;

    use quartz_nbt::snbt;
    use crate::{data_converter_func, DataConverter, DataType, DataVersion, MapDataType, MapType, ObjectType, QuartzNbtTypes, Types};

    fn make_map(string: &str) -> <TestTypes as Types>::Map {
        snbt::parse(string).expect("snbt syntax error")
    }

    #[test]
    fn rename_key() {
        rename_key_0::<TestTypes>(make_map(r#"{"hello": "world"}"#));
    }

    fn rename_key_0<T: Types + ?Sized>(mut map: T::Map) {
        map.rename_key("hello", "Hello");
        assert!(map.has_key("Hello"));
        assert!(!map.has_key("hello"));
    }

    #[test]
    fn simple_conversion() {
        simple_conversion_0::<TestTypes>(make_map(r#"{"test": 42}"#));
    }

    fn simple_conversion_0<T: Types + ?Sized>(mut map: T::Map) {
        let mut typ = MapDataType::<T>::new("Test");
        typ.add_structure_converter(1, data_converter_func::<T::Map, _>(|data, from_version, to_version| {
            data.set("test", T::Object::create_string(data.get_i64("test").unwrap().to_string()))
        }));
        typ.convert(&mut map, 0.into(), 1.into());
        assert_eq!("42", map.get_string("test").unwrap());
    }
}
