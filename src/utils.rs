use crate::{
    AbstractDynamicDataType, AbstractMapDataType, AbstractValueDataType, DataVersion, JCompound,
    JList, JValue, MapDataWalker,
};
use java_string::{JavaStr, JavaString};
use log::warn;

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

impl<T> MapDataWalker for DataWalkerObjectListPaths<T>
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

impl<T> MapDataWalker for DataWalkerMapListPaths<T>
where
    T: AbstractMapDataType,
{
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_list_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerDynamicListPaths<T>
where
    T: AbstractDynamicDataType,
{
    typ: T,
    paths: Vec<String>,
}

impl<T> DataWalkerDynamicListPaths<T>
where
    T: AbstractDynamicDataType,
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { typ, paths }
    }
}

impl<T> MapDataWalker for DataWalkerDynamicListPaths<T>
where
    T: AbstractDynamicDataType,
{
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_dynamic_list_in_map(&self.typ, data, path, from_version, to_version);
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

impl<T> MapDataWalker for DataWalkerObjectTypePaths<T>
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

impl<T> MapDataWalker for DataWalkerMapTypePaths<T>
where
    T: AbstractMapDataType,
{
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_in_map(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerDynamicTypePaths<T>
where
    T: AbstractDynamicDataType,
{
    typ: T,
    paths: Vec<String>,
}

impl<T> DataWalkerDynamicTypePaths<T>
where
    T: AbstractDynamicDataType,
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { typ, paths }
    }
}

impl<T> MapDataWalker for DataWalkerDynamicTypePaths<T>
where
    T: AbstractDynamicDataType,
{
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_dynamic_in_map(&self.typ, data, path, from_version, to_version);
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

pub fn convert_dynamic_in_map<T>(
    data_type: T,
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractDynamicDataType,
{
    if let Some(value) = data.get_mut(path.as_ref()) {
        data_type.convert(value, from_version, to_version);
    }
}

pub fn convert_dynamic_list_in_map<T>(
    data_type: T,
    data: &mut JCompound,
    path: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) where
    T: AbstractDynamicDataType,
{
    fn convert_list_inner<T: AbstractDynamicDataType, E: Into<JValue>>(
        data_type: T,
        in_list: &mut Vec<E>,
        from_version: DataVersion,
        to_version: DataVersion,
    ) -> JList {
        let mut result = JList::new();
        let mut all_success = true;
        for element in in_list.drain(..) {
            let mut element = element.into();
            data_type.convert(&mut element, from_version, to_version);
            all_success &= result.try_push(element)
        }
        if !all_success {
            warn!("Result of list conversion was not homogenous");
        }
        result
    }

    let Some(valence_nbt::Value::List(list)) = data.get_mut(path.as_ref()) else {
        return;
    };
    *list = match list {
        valence_nbt::List::End => JList::End,
        valence_nbt::List::Byte(bytes) => {
            convert_list_inner(data_type, bytes, from_version, to_version)
        }
        valence_nbt::List::Short(shorts) => {
            convert_list_inner(data_type, shorts, from_version, to_version)
        }
        valence_nbt::List::Int(ints) => {
            convert_list_inner(data_type, ints, from_version, to_version)
        }
        valence_nbt::List::Long(longs) => {
            convert_list_inner(data_type, longs, from_version, to_version)
        }
        valence_nbt::List::Float(floats) => {
            convert_list_inner(data_type, floats, from_version, to_version)
        }
        valence_nbt::List::Double(doubles) => {
            convert_list_inner(data_type, doubles, from_version, to_version)
        }
        valence_nbt::List::ByteArray(byte_arrays) => {
            convert_list_inner(data_type, byte_arrays, from_version, to_version)
        }
        valence_nbt::List::String(strings) => {
            convert_list_inner(data_type, strings, from_version, to_version)
        }
        valence_nbt::List::List(lists) => {
            convert_list_inner(data_type, lists, from_version, to_version)
        }
        valence_nbt::List::Compound(compounds) => {
            convert_list_inner(data_type, compounds, from_version, to_version)
        }
        valence_nbt::List::IntArray(int_arrays) => {
            convert_list_inner(data_type, int_arrays, from_version, to_version)
        }
        valence_nbt::List::LongArray(long_arrays) => {
            convert_list_inner(data_type, long_arrays, from_version, to_version)
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
