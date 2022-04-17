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
    use crate::{convert_map_in_map, convert_object_in_map, data_converter_func, data_walker, DataType, IdDataType, MapDataType, MapType, ObjectDataType, ObjectType, QuartzNbtTypes, Types};

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
        simple_converted_type::<T>().convert(&mut map, 0.into(), 1.into());
        assert_eq!("42", map.get_string("test").unwrap());
    }

    #[test]
    fn simple_walker() {
        simple_walker_0::<TestTypes>(make_map(r#"{"inner": {"test": 42}}"#))
    }

    fn simple_walker_0<T: Types + ?Sized>(mut map: T::Map) {
        let mut typ = MapDataType::<T>::new("Outer");
        let inner_type = simple_converted_type::<T>();
        typ.add_structure_walker(1, data_walker::<T, _>(move |data, from_version, to_version| {
            convert_map_in_map::<_, T>(&inner_type, data, "inner", from_version, to_version)
        }));
        typ.convert(&mut map, 0.into(), 1.into());
        assert_eq!("42", map.get_map("inner").unwrap().get_string("test").unwrap());
    }

    #[test]
    fn simple_id_walker() {
        simple_id_walker_0::<TestTypes>(make_map(r#"{"id": "foo", "test": 42}"#), make_map(r#"{"id": "bar", "test": 42}"#))
    }

    fn simple_id_walker_0<T: Types + ?Sized>(mut map1: T::Map, mut map2: T::Map) {
        let mut inner_type = ObjectDataType::<T>::new("Inner");
        inner_type.add_structure_converter(1, data_converter_func::<T::Object, _>(|data, _from_version, _to_version| {
            if let Some(i) = data.as_i64() {
                *data = T::Object::create_string(i.to_string())
            }
        }));
        let mut typ = IdDataType::<T>::new("Test");
        typ.add_walker_for_id(1, "foo", data_walker::<T, _>(move |data, from_version, to_version| {
            convert_object_in_map::<_, T>(&inner_type, data, "test", from_version, to_version);
        }));

        typ.convert(&mut map1, 0.into(), 1.into());
        typ.convert(&mut map2, 0.into(), 1.into());
        assert_eq!("42", map1.get_string("test").unwrap());
        assert_eq!(42, map2.get_i64("test").unwrap());
    }

    fn simple_converted_type<'a, T: Types + ?Sized>() -> MapDataType<'a, T> {
        let mut ret = MapDataType::new("Test");
        ret.add_structure_converter(1, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            if let Some(i) = data.get_i64("test") {
                data.set("test", T::Object::create_string(i.to_string()));
            }
        }));
        ret
    }
}
