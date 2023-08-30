mod convert;
mod utils;

pub use crate::convert::*;
pub use crate::utils::*;

#[cfg(test)]
mod tests {
    use crate::{
        convert_map_in_map, convert_object_in_map, data_walker, map_data_converter_func,
        value_data_converter_func, AbstractMapDataType, IdDataType, MapDataType, ObjectDataType,
    };
    use valence_nbt::Compound;

    fn make_map(string: &str) -> Compound {
        let value = valence_nbt::snbt::from_snbt_str(string).expect("snbt syntax error");
        match value {
            valence_nbt::Value::Compound(compound) => compound,
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
        simple_converted_type().convert((), &mut map, 0.into(), 1.into());
        assert!(matches!(map.get("test"), Some(valence_nbt::Value::String(str)) if str == "42"));
    }

    #[test]
    fn simple_walker() {
        let mut map = make_map(r#"{"inner": {"test": 42}}"#);
        let mut typ = MapDataType::new("Outer");
        let inner_type = simple_converted_type();
        typ.add_structure_walker(
            1,
            data_walker(move |context, data, from_version, to_version| {
                convert_map_in_map(&inner_type, context, data, "inner", from_version, to_version)
            }),
        );
        typ.convert((), &mut map, 0.into(), 1.into());
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
            value_data_converter_func(|_context, data, _from_version, _to_version| {
                if let valence_nbt::value::ValueMut::Int(ref mut i) = data {
                    **i = 69;
                }
            }),
        );
        let mut typ = IdDataType::new("Test");
        typ.add_walker_for_id(
            1,
            "foo",
            data_walker(move |context, data, from_version, to_version| {
                convert_object_in_map(&inner_type, context, data, "test", from_version, to_version);
            }),
        );

        typ.convert((), &mut map1, 0.into(), 1.into());
        typ.convert((), &mut map2, 0.into(), 1.into());
        assert_eq!(69, map1.get("test").unwrap().as_i64().unwrap());
        assert_eq!(42, map2.get("test").unwrap().as_i64().unwrap());
    }

    fn simple_converted_type() -> MapDataType<()> {
        let mut ret = MapDataType::new("Test");
        ret.add_structure_converter(
            1,
            map_data_converter_func(|_context, data, _from_version, _to_version| {
                if let Some(valence_nbt::Value::Int(i)) = data.get("test") {
                    data.insert("test", valence_nbt::Value::String(i.to_string()));
                }
            }),
        );
        ret
    }
}
