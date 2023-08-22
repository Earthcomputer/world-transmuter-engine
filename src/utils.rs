use crate::{AbstractMapDataType, AbstractValueDataType, DataVersion, DataWalker};
use valence_nbt::Compound;

pub struct DataWalkerObjectListPaths<T>
where
    T: AbstractValueDataType,
{
    typ: T,
    paths: Vec<String>,
}

impl<T> DataWalkerObjectListPaths<T>
where
    T: AbstractValueDataType,
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { typ, paths }
    }
}

impl<T> DataWalker for DataWalkerObjectListPaths<T>
where
    T: AbstractValueDataType,
{
    fn walk(&self, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_object_list_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerMapListPaths<T>
where
    T: AbstractMapDataType,
{
    typ: T,
    paths: Vec<String>,
}

impl<T> DataWalkerMapListPaths<T>
where
    T: AbstractMapDataType,
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { typ, paths }
    }
}

impl<T> DataWalker for DataWalkerMapListPaths<T>
where
    T: AbstractMapDataType,
{
    fn walk(&self, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_list_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerObjectTypePaths<T>
where
    T: AbstractValueDataType,
{
    typ: T,
    paths: Vec<String>,
}

impl<T> DataWalkerObjectTypePaths<T>
where
    T: AbstractValueDataType,
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { typ, paths }
    }
}

impl<T> DataWalker for DataWalkerObjectTypePaths<T>
where
    T: AbstractValueDataType,
{
    fn walk(&self, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_object_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerMapTypePaths<T>
where
    T: AbstractMapDataType,
{
    typ: T,
    paths: Vec<String>,
}

impl<T> DataWalkerMapTypePaths<T>
where
    T: AbstractMapDataType,
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { typ, paths }
    }
}

impl<T> DataWalker for DataWalkerMapTypePaths<T>
where
    T: AbstractMapDataType,
{
    fn walk(&self, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub fn convert_map_in_map<T>(
    data_type: T,
    data: &mut Compound,
    path: &str,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    if let Some(valence_nbt::Value::Compound(map)) = data.get_mut(path) {
        data_type.convert(map, from_version, to_version);
    }
}

pub fn convert_map_list_in_map<T>(
    data_type: T,
    data: &mut Compound,
    path: &str,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    if let Some(valence_nbt::Value::List(valence_nbt::List::Compound(list))) = data.get_mut(path) {
        for map in list {
            data_type.convert(map, from_version, to_version);
        }
    }
}

pub fn convert_object_in_map<T>(
    data_type: T,
    data: &mut Compound,
    path: &str,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractValueDataType,
{
    if let Some(obj) = data.get_mut(path) {
        data_type.convert(&mut obj.as_value_mut(), from_version, to_version);
    }
}

pub fn convert_object_list<T>(
    data_type: T,
    data: &mut valence_nbt::List,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractValueDataType,
{
    for mut obj in data.iter_mut() {
        data_type.convert(&mut obj, from_version, to_version);
    }
}

pub fn convert_object_list_in_map<T>(
    data_type: T,
    data: &mut Compound,
    path: &str,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractValueDataType,
{
    if let Some(valence_nbt::Value::List(list)) = data.get_mut(path) {
        for mut obj in list.iter_mut() {
            data_type.convert(&mut obj, from_version, to_version);
        }
    }
}

pub fn convert_values_in_map<T>(
    data_type: T,
    data: &mut Compound,
    path: &str,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    if let Some(valence_nbt::Value::Compound(map)) = data.get_mut(path) {
        convert_values(data_type, map, from_version, to_version);
    }
}

pub fn convert_values<T>(
    data_type: T,
    data: &mut Compound,
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    for obj in data.values_mut() {
        if let valence_nbt::Value::Compound(map) = obj {
            data_type.convert(map, from_version, to_version);
        }
    }
}

pub fn rename_key(map: &mut Compound, from: &str, to: impl Into<String>) {
    if let Some(value) = map.remove(from) {
        map.insert(to.into(), value);
    }
}
