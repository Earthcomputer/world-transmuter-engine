use std::marker::PhantomData;
use crate::{DataType, DataVersion, ListType, MapType, Types, ObjectType, DataWalker};

pub struct DataWalkerObjectListPaths<T, U>
    where T: DataType<U::Object>, U: Types + ?Sized
{
    phantom: PhantomData<U>,
    typ: T,
    paths: Vec<String>,
}

impl<'a, T, U> DataWalkerObjectListPaths<T, U>
    where T: DataType<U::Object>, U: Types + ?Sized
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { phantom: PhantomData, typ, paths }
    }
}

impl<T, U> DataWalker<U> for DataWalkerObjectListPaths<T, U>
    where T: DataType<U::Object>, U: Types + ?Sized
{
    fn walk(&self, data: &mut U::Map, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_object_list_in_map::<_, U>(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerMapListPaths<T, U>
    where T: DataType<U::Map>, U: Types + ?Sized
{
    phantom: PhantomData<U>,
    typ: T,
    paths: Vec<String>,
}

impl<T, U> DataWalkerMapListPaths<T, U>
    where T: DataType<U::Map>, U: Types + ?Sized
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { phantom: PhantomData, typ, paths }
    }
}

impl<T, U> DataWalker<U> for DataWalkerMapListPaths<T, U>
    where T: DataType<U::Map>, U: Types + ?Sized
{
    fn walk(&self, data: &mut U::Map, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_list_in_map::<_, U>(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerObjectTypePaths<T, U>
    where T: DataType<U::Object>, U: Types + ?Sized
{
    phantom: PhantomData<U>,
    typ: T,
    paths: Vec<String>,
}

impl<T, U> DataWalkerObjectTypePaths<T, U>
    where T: DataType<U::Object>, U: Types + ?Sized
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { phantom: PhantomData, typ, paths }
    }
}

impl<T, U> DataWalker<U> for DataWalkerObjectTypePaths<T, U>
    where T: DataType<U::Object>, U: Types + ?Sized
{
    fn walk(&self, data: &mut U::Map, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_object_in_map::<_, U>(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub struct DataWalkerMapTypePaths<T, U>
    where T: DataType<U::Map>, U: Types + ?Sized
{
    phantom: PhantomData<U>,
    typ: T,
    paths: Vec<String>,
}

impl<T, U> DataWalkerMapTypePaths<T, U>
    where T: DataType<U::Map>, U: Types + ?Sized
{
    pub fn new(typ: T, path: impl Into<String>) -> Self {
        Self::new_multi(typ, vec![path.into()])
    }

    pub fn new_multi(typ: T, paths: Vec<String>) -> Self {
        Self { phantom: PhantomData, typ, paths }
    }
}

impl<T, U> DataWalker<U> for DataWalkerMapTypePaths<T, U>
    where T: DataType<U::Map>, U: Types + ?Sized
{
    fn walk(&self, data: &mut U::Map, from_version: DataVersion, to_version: DataVersion) {
        for path in &self.paths {
            convert_map_in_map::<_, U>(&self.typ, data, path, from_version, to_version);
        }
    }
}

pub fn convert_map_in_map<T, U>(data_type: T, data: &mut U::Map, path: &str, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Map>, U: Types + ?Sized
{
    if let Some(map) = data.get_map_mut(path) {
        data_type.convert(map, from_version, to_version);
    }
}

pub fn convert_map_list_in_map<T, U>(data_type: T, data: &mut U::Map, path: &str, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Map>, U: Types + ?Sized
{
    if let Some(list) = data.get_list_mut(path) {
        for value in list.iter_mut() {
            if let Some(map) = value.as_map_mut() {
                data_type.convert(map, from_version, to_version);
            }
        }
    }
}

pub fn convert_object_in_map<T, U>(data_type: T, data: &mut U::Map, path: &str, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Object>, U: Types + ?Sized
{
    if let Some(obj) = data.get_mut(path) {
        data_type.convert(obj, from_version, to_version);
    }
}

pub fn convert_object_list<T, U>(data_type: T, data: &mut U::List, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Object>, U: Types + ?Sized
{
    for obj in data.iter_mut() {
        data_type.convert(obj, from_version, to_version);
    }
}

pub fn convert_object_list_in_map<T, U>(data_type: T, data: &mut U::Map, path: &str, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Object>, U: Types + ?Sized
{
    if let Some(list) = data.get_list_mut(path) {
        for obj in list.iter_mut() {
            data_type.convert(obj, from_version, to_version);
        }
    }
}

pub fn convert_values_in_map<T, U>(data_type: T, data: &mut U::Map, path: &str, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Map>, U: Types + ?Sized
{
    if let Some(map) = data.get_map_mut(path) {
        convert_values::<T, U>(data_type, map, from_version, to_version);
    }
}

pub fn convert_values<T, U>(data_type: T, data: &mut U::Map, from_version: DataVersion, to_version: DataVersion)
    where T: DataType<U::Map>, U: Types + ?Sized
{
    for obj in data.values_mut() {
        if let Some(map) = obj.as_map_mut() {
            data_type.convert(map, from_version, to_version);
        }
    }
}
