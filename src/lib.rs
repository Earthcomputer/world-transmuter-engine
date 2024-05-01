mod convert;
mod utils;

pub use crate::convert::*;
pub use crate::utils::*;
use java_string::JavaString;

pub type JCompound = valence_nbt::Compound<JavaString>;
pub type JList = valence_nbt::List<JavaString>;
pub type JValue = valence_nbt::Value<JavaString>;
pub type JValueRef<'a> = valence_nbt::value::ValueRef<'a, JavaString>;
pub type JValueMut<'a> = valence_nbt::value::ValueMut<'a, JavaString>;

pub fn value_to_java(value: valence_nbt::Value) -> JValue {
    match value {
        valence_nbt::Value::Byte(v) => JValue::Byte(v),
        valence_nbt::Value::Short(v) => JValue::Short(v),
        valence_nbt::Value::Int(v) => JValue::Int(v),
        valence_nbt::Value::Long(v) => JValue::Long(v),
        valence_nbt::Value::Float(v) => JValue::Float(v),
        valence_nbt::Value::Double(v) => JValue::Double(v),
        valence_nbt::Value::ByteArray(v) => JValue::ByteArray(v),
        valence_nbt::Value::String(v) => JValue::String(JavaString::from(v)),
        valence_nbt::Value::List(v) => JValue::List(list_to_java(v)),
        valence_nbt::Value::Compound(v) => JValue::Compound(compound_to_java(v)),
        valence_nbt::Value::IntArray(v) => JValue::IntArray(v),
        valence_nbt::Value::LongArray(v) => JValue::LongArray(v),
    }
}

fn list_to_java(list: valence_nbt::List) -> JList {
    match list {
        valence_nbt::List::End => JList::End,
        valence_nbt::List::Byte(v) => JList::Byte(v),
        valence_nbt::List::Short(v) => JList::Short(v),
        valence_nbt::List::Int(v) => JList::Int(v),
        valence_nbt::List::Long(v) => JList::Long(v),
        valence_nbt::List::Float(v) => JList::Float(v),
        valence_nbt::List::Double(v) => JList::Double(v),
        valence_nbt::List::ByteArray(v) => JList::ByteArray(v),
        valence_nbt::List::String(v) => {
            JList::String(v.into_iter().map(JavaString::from).collect())
        }
        valence_nbt::List::List(v) => JList::List(v.into_iter().map(list_to_java).collect()),
        valence_nbt::List::Compound(v) => {
            JList::Compound(v.into_iter().map(compound_to_java).collect())
        }
        valence_nbt::List::IntArray(v) => JList::IntArray(v),
        valence_nbt::List::LongArray(v) => JList::LongArray(v),
    }
}

pub fn compound_to_java(compound: valence_nbt::Compound) -> JCompound {
    let mut result = JCompound::with_capacity(compound.len());
    for (key, value) in compound {
        result.insert(JavaString::from(key), value_to_java(value));
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{
        convert_map_in_map, convert_object_in_map, map_data_converter_func, map_data_walker,
        value_data_converter_func, value_to_java, AbstractMapDataType, IdDataType, JCompound,
        JValue, MapDataType, ObjectDataType,
    };
    use java_string::JavaString;

    fn make_map(string: &str) -> JCompound {
        let value =
            value_to_java(valence_nbt::snbt::from_snbt_str(string).expect("snbt syntax error"));
        match value {
            JValue::Compound(compound) => compound,
            _ => panic!("snbt was not a compound"),
        }
    }

    #[test]
    fn rename_key() {
        let mut map = make_map(r#"{"hello": "world"}"#);
        crate::rename_key(&mut map, "hello", "Hello");
        assert!(map.contains_key("Hello"));
        assert!(!map.contains_key("hello"));
    }

    #[test]
    fn simple_conversion() {
        let mut map = make_map(r#"{"test": 42}"#);
        simple_converted_type().convert(&mut map, 0.into(), 1.into());
        assert!(matches!(map.get("test"), Some(valence_nbt::Value::String(str)) if str == "42"));
    }

    #[test]
    fn simple_walker() {
        let mut map = make_map(r#"{"inner": {"test": 42}}"#);
        let mut typ = MapDataType::new("Outer");
        let inner_type = simple_converted_type();
        typ.add_structure_walker(
            1,
            map_data_walker(move |data, from_version, to_version| {
                convert_map_in_map(&inner_type, data, "inner", from_version, to_version)
            }),
        );
        typ.convert(&mut map, 0.into(), 1.into());
        assert!(
            matches!(map.get("inner"), Some(valence_nbt::Value::Compound(inner)) if matches!(inner.get("test"), Some(valence_nbt::Value::String(str)) if str == "42"))
        );
    }

    #[test]
    fn simple_id_walker() {
        let mut map1 = make_map(r#"{"id": "foo", "test": 42}"#);
        let mut map2 = make_map(r#"{"id": "bar", "test": 42}"#);
        let mut inner_type = ObjectDataType::new("Inner");
        inner_type.add_structure_converter(
            1,
            value_data_converter_func(|data, _from_version, _to_version| {
                if let valence_nbt::value::ValueMut::Int(i) = data {
                    **i = 69;
                }
            }),
        );
        let mut typ = IdDataType::new("Test");
        typ.add_walker_for_id(
            1,
            "foo",
            map_data_walker(move |data, from_version, to_version| {
                convert_object_in_map(&inner_type, data, "test", from_version, to_version);
            }),
        );

        typ.convert(&mut map1, 0.into(), 1.into());
        typ.convert(&mut map2, 0.into(), 1.into());
        assert_eq!(69, map1.get("test").unwrap().as_i64().unwrap());
        assert_eq!(42, map2.get("test").unwrap().as_i64().unwrap());
    }

    fn simple_converted_type() -> MapDataType<'static> {
        let mut ret = MapDataType::new("Test");
        ret.add_structure_converter(
            1,
            map_data_converter_func(|data, _from_version, _to_version| {
                if let Some(JValue::Int(i)) = data.get("test") {
                    data.insert("test", JValue::String(JavaString::from(i.to_string())));
                }
            }),
        );
        ret
    }
}
