use std::cmp::Ordering;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::{MapType, ObjectRef, ObjectType, Types};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DataVersion {
    version: u32,
    step: u32,
}

impl DataVersion {
    pub fn new(version: u32, step: u32) -> Self {
        Self { version, step }
    }

    #[inline]
    pub fn get_version(&self) -> u32 {
        self.version
    }

    #[inline]
    pub fn get_step(&self) -> u32 {
        self.step
    }
}

pub trait DataConverterFunc<T> {
    fn convert(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion);
}

impl<U, T: DataConverterFunc<U> + ?Sized> DataConverterFunc<U> for &T {
    fn convert(&self, data: &mut U, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<U, T: DataConverterFunc<U> + ?Sized> DataConverterFunc<U> for Box<T> {
    fn convert(&self, data: &mut U, from_version: DataVersion, to_version: DataVersion) {
        T::convert(&*self, data, from_version, to_version)
    }
}

pub struct DataConverter<T, F: DataConverterFunc<T>> {
    typ: PhantomData<T>,
    to_version: DataVersion,
    conversion_func: F,
}

impl<T, F: DataConverterFunc<T>> DataConverter<T, F> {
    pub fn new(to_version: DataVersion, conversion_func: F) -> Self {
        Self { typ: PhantomData, to_version, conversion_func }
    }

    #[inline]
    pub fn get_to_version(&self) -> DataVersion {
        self.to_version
    }

    pub fn convert(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion) {
        self.conversion_func.convert(data, from_version, to_version)
    }
}

impl<T, F: DataConverterFunc<T>> core::fmt::Debug for DataConverter<T, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("DataConverter{{{:?}}}", self.to_version))
    }
}

impl<T, F: DataConverterFunc<T>> PartialEq for DataConverter<T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.to_version == other.to_version
    }
}

impl<T, F: DataConverterFunc<T>> Eq for DataConverter<T, F> {}

impl<T, F: DataConverterFunc<T>> PartialOrd for DataConverter<T, F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.to_version.partial_cmp(&other.to_version)
    }
}

impl<T, F: DataConverterFunc<T>> Ord for DataConverter<T, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_version.cmp(&other.to_version)
    }
}

pub struct ConversionError {
    pub message: String,
}

pub type Result<T> = core::result::Result<T, ConversionError>;

pub trait DataType<T> {
    fn convert(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion);
}

impl<T, U: DataType<T>> DataType<T> for &U {
    fn convert(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion) {
        U::convert(self, data, from_version, to_version)
    }
}

macro_rules! structure_converters {
    ($ty:ident, $field_name:ident, $converted_type:ty) => {
        impl<T: Types + ?Sized> $ty<T> {
            pub fn add_structure_converter(&mut self, converter: $converted_type) {
                let index = self.$field_name.binary_search(&converter);
                let index = match index {
                    Ok(i) => i,
                    Err(i) => i,
                };
                self.$field_name.insert(index, converter);
            }
        }
    }
}

macro_rules! version_list {
    ($ty:ident, $method_name:ident, $field_name:ident, $element_type:ty) => {
        impl<T: Types + ?Sized> $ty<T> {
            pub fn $method_name(&mut self, version: DataVersion, value: $element_type) {
                self.$field_name.entry(version).or_default().push(value);
            }
        }
    }
}

pub struct MapDataType<T: Types + ?Sized> {
    pub name: String,
    structure_converters: Vec<DataConverter<T::Map, Box<dyn DataConverterFunc<T::Map>>>>,
    structure_walkers: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataWalker<T>>>>,
    structure_hooks: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataHook<T::Map>>>>,
}
structure_converters!(MapDataType, structure_converters, DataConverter<T::Map, Box<dyn DataConverterFunc<T::Map>>>);
version_list!(MapDataType, add_structure_walker, structure_walkers, Box<dyn DataWalker<T>>);
version_list!(MapDataType, add_structure_hook, structure_hooks, Box<dyn DataHook<T::Map>>);
impl<T: Types + ?Sized> MapDataType<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            structure_converters: Vec::new(),
            structure_walkers: std::collections::BTreeMap::new(),
            structure_hooks: std::collections::BTreeMap::new(),
        }
    }
}

impl<T: Types + ?Sized> DataType<T::Map> for MapDataType<T> {
    fn convert(&self, data: &mut T::Map, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.structure_converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self.structure_hooks.range(..=converter.get_to_version()).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks {
                    hook.pre_hook(data, from_version, to_version);
                }
            }

            converter.convert(data, from_version, to_version);

            // possibly new data format, update hooks
            let hooks = self.structure_hooks.range(..=to_version).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks.iter().rev() {
                    hook.post_hook(data, from_version, to_version);
                }
            }
        }

        let hooks = self.structure_hooks.range(..=to_version).next_back();
        if let Some((_, hooks)) = hooks {
            for hook in hooks {
                hook.pre_hook(data, from_version, to_version);
            }
        }

        let walkers = self.structure_walkers.range(..=to_version).next_back();
        if let Some((_, walkers)) = walkers {
            for walker in walkers {
                walker.walk(data, from_version, to_version);
            }
        }

        if let Some((_, hooks)) = hooks {
            for hook in hooks.iter().rev() {
                hook.post_hook(data, from_version, to_version);
            }
        }
    }
}

pub struct ObjectDataType<T: Types + ?Sized> {
    pub name: String,
    converters: Vec<DataConverter<T::Object, Box<dyn DataConverterFunc<T::Object>>>>,
    structure_hooks: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataHook<T::Object>>>>,
}
structure_converters!(ObjectDataType, converters, DataConverter<T::Object, Box<dyn DataConverterFunc<T::Object>>>);
version_list!(ObjectDataType, add_structure_hook, structure_hooks, Box<dyn DataHook<T::Object>>);

impl<T: Types + ?Sized> ObjectDataType<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            converters: Vec::new(),
            structure_hooks: std::collections::BTreeMap::new(),
        }
    }
}

impl<T: Types + ?Sized> DataType<T::Object> for ObjectDataType<T> {
    fn convert(&self, data: &mut T::Object, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self.structure_hooks.range(..=converter.get_to_version()).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks {
                    hook.pre_hook(data, from_version, to_version);
                }
            }

            converter.convert(data, from_version, to_version);

            // possibly new data format, update hooks
            let hooks = self.structure_hooks.range(..=to_version).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks.iter().rev() {
                    hook.post_hook(data, from_version, to_version);
                }
            }
        }
    }
}

pub struct IdDataType<T: Types + ?Sized> {
    pub name: String,
    structure_converters: Vec<DataConverter<T::Map, Box<dyn DataConverterFunc<T::Map>>>>,
    structure_walkers: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataWalker<T>>>>,
    structure_hooks: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataHook<T::Map>>>>,
    walkers_by_id: crate::Map<String, std::collections::BTreeMap<DataVersion, Vec<Rc<dyn DataWalker<T>>>>>,
}
structure_converters!(IdDataType, structure_converters, DataConverter<T::Map, Box<dyn DataConverterFunc<T::Map>>>);
version_list!(IdDataType, add_structure_walker, structure_walkers, Box<dyn DataWalker<T>>);
version_list!(IdDataType, add_structure_hook, structure_hooks, Box<dyn DataHook<T::Map>>);

impl<T: 'static + Types + ?Sized> IdDataType<T> {
    fn new(name: String) -> Self {
        Self {
            name,
            structure_converters: Vec::new(),
            structure_walkers: std::collections::BTreeMap::new(),
            structure_hooks: std::collections::BTreeMap::new(),
            walkers_by_id: crate::Map::new(),
        }
    }

    fn add_converter_for_id(&mut self, id: String, converter: DataConverter<T::Map, Box<dyn DataConverterFunc<T::Map>>>) {
        struct ConvertedForId<T: Types + ?Sized>(String, Box<dyn DataConverterFunc<T::Map>>);
        impl<T: Types + ?Sized> DataConverterFunc<T::Map> for ConvertedForId<T> {
            fn convert(&self, data: &mut T::Map, from_version: DataVersion, to_version: DataVersion) {
                if data.get_string("id") == Some(&self.0) {
                    self.1.convert(data, from_version, to_version)
                }
            }
        }
        self.add_structure_converter(DataConverter::new(
            converter.get_to_version(),
            Box::new(ConvertedForId::<T>(id, converter.conversion_func))
        ));
    }

    fn add_walker_for_id(&mut self, version: DataVersion, id: String, walker: Rc<dyn DataWalker<T>>) {
        self.walkers_by_id.entry(id).or_default().entry(version).or_default().push(walker);
    }

    fn copy_walkers(&mut self, version: DataVersion, from_id: &str, to_id: String) {
        if let Some(from_versions) = self.walkers_by_id.get(from_id) {
            if let Some((_, from_walkers)) = from_versions.range(..=version).next_back() {
                for walker in from_walkers.clone() {
                    self.add_walker_for_id(version, to_id.clone(), walker);
                }
            }
        }
    }
}

impl<T: Types + ?Sized> DataType<T::Map> for IdDataType<T> {
    fn convert(&self, data: &mut T::Map, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.structure_converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self.structure_hooks.range(..=converter.get_to_version()).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks {
                    hook.pre_hook(data, from_version, to_version);
                }
            }

            converter.convert(data, from_version, to_version);

            // possibly new data format, update hooks
            let hooks = self.structure_hooks.range(..=to_version).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks {
                    hook.post_hook(data, from_version, to_version);
                }
            }
        }

        // run pre hooks

        let hooks = self.structure_hooks.range(..=to_version).next_back();
        if let Some((_, hooks)) = hooks {
            for hook in hooks.iter().rev() {
                hook.pre_hook(data, from_version, to_version);
            }
        }

        // run all walkers

        let walkers = self.structure_walkers.range(..=to_version).next_back();
        if let Some((_, walkers)) = walkers {
            for walker in walkers {
                walker.walk(data, from_version, to_version);
            }
        }

        if let Some(id) = data.get_string("id") {
            if let Some(walkers_by_version) = self.walkers_by_id.get(id) {
                if let Some((_, walkers)) = walkers_by_version.range(..=to_version).next_back() {
                    for walker in walkers {
                        walker.walk(data, from_version, to_version);
                    }
                }
            }
        }

        // run post hooks

        if let Some((_, hooks)) = hooks {
            for hook in hooks.iter().rev() {
                hook.post_hook(data, from_version, to_version);
            }
        }
    }
}

pub trait DataHook<T> {
    fn pre_hook(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion);
    fn post_hook(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion);
}

pub trait DataWalker<T: Types + ?Sized> {
    fn walk(&self, data: &mut T::Map, from_version: DataVersion, to_version: DataVersion);
}
