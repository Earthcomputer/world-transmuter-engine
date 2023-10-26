use crate::{
    AbstractMapDataType, AbstractValueDataType, DataVersion, DataWalker, JCompound, JList, JValue,
};
use java_string::{JavaStr, JavaString};

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
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
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
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
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
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
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
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub fn convert_map_in_map<T>(
    data_type: T,
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    if let Some(valence_nbt::Value::Compound(map)) = data.get_mut(path.as_ref()) {
        data_type.convert(map, from_version, to_version);
    }
}

pub fn convert_map_list_in_map<T>(
    data_type: T,
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    if let Some(valence_nbt::Value::List(valence_nbt::List::Compound(list))) =
        data.get_mut(path.as_ref())
    {
        for map in list {
            data_type.convert(map, from_version, to_version);
        }
    }
}

pub fn convert_object_in_map<T>(
    data_type: T,
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractValueDataType,
{
    if let Some(obj) = data.get_mut(path.as_ref()) {
        data_type.convert(&mut obj.as_value_mut(), from_version, to_version);
    }
}

pub fn convert_object_list<T>(
    data_type: T,
    data: &mut JList,
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
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractValueDataType,
{
    if let Some(valence_nbt::Value::List(list)) = data.get_mut(path.as_ref()) {
        for mut obj in list.iter_mut() {
            data_type.convert(&mut obj, from_version, to_version);
        }
    }
}

pub fn convert_values_in_map<T>(
    data_type: T,
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractMapDataType,
{
    if let Some(valence_nbt::Value::Compound(map)) = data.get_mut(path.as_ref()) {
        convert_values(data_type, map, from_version, to_version);
    }
}

pub fn convert_values<T>(
    data_type: T,
    data: &mut JCompound,
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

#[inline]
pub fn rename_key(map: &mut JCompound, from: impl AsRef<JavaStr>, to: impl Into<JavaString>) {
    if let Some(value) = map.remove(from.as_ref()) {
        map.insert(to.into(), value);
    }
}

pub fn rename_keys(map: &mut JCompound, renamer: impl Fn(&JavaStr) -> Option<JavaString>) {
    let renames: Vec<_> = map
        .keys()
        .filter_map(|key| renamer(key).map(|new_key| (key.clone(), new_key)))
        .collect();
    let renamed: Vec<_> = renames
        .into_iter()
        .map(|(from, to)| (to, map.remove(&from[..]).unwrap()))
        .collect();
    for (key, value) in renamed {
        map.insert(key, value);
    }
}

pub fn get_mut_multi<'a, const N: usize>(
    map: &'a mut JCompound,
    keys: [&str; N],
) -> [Option<&'a mut JValue>; N] {
    #[cold]
    #[inline(never)]
    fn non_unique_keys(keys: &[&str]) -> ! {
        panic!("keys are not all unique: {keys:?}")
    }

    if N > 1 {
        for i in 0..N - 1 {
            for j in i + 1..N {
                if keys[i] == keys[j] {
                    non_unique_keys(&keys);
                }
            }
        }
    }

    keys.map(|key| {
        map.get_mut(key).map(|value| {
            // SAFETY: we just checked that all keys are unique, so these mutable references are all different values in the map, so they can coexist.
            unsafe { &mut *(value as *mut _) }
        })
    })
}
