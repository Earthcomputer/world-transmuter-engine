use std::cmp::Ordering;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::{MapType, Types};

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

impl From<u32> for DataVersion {
    fn from(value: u32) -> Self {
        DataVersion::new(value, 0)
    }
}

pub trait DataConverterFunc<T> {
    fn convert(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion);
}

pub fn data_converter_func<T, F>(func: F) -> impl DataConverterFunc<T>
    where F: Fn(&mut T, DataVersion, DataVersion)
{
    struct DataConverterFuncImpl<F>(F);
    impl<T, F> DataConverterFunc<T> for DataConverterFuncImpl<F>
        where F: Fn(&mut T, DataVersion, DataVersion)
    {
        fn convert(&self, data: &mut T, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(data, from_version, to_version)
        }
    }
    DataConverterFuncImpl(func)
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
    pub fn new(to_version: impl Into<DataVersion>, conversion_func: F) -> Self {
        Self { typ: PhantomData, to_version: to_version.into(), conversion_func }
    }

    #[inline]
    pub fn get_to_version(&self) -> DataVersion {
        self.to_version
    }

    pub fn convert(&self, data: &mut T, from_version: impl Into<DataVersion>, to_version: impl Into<DataVersion>) {
        self.conversion_func.convert(data, from_version.into(), to_version.into())
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
            pub fn add_structure_converter(&mut self, version: impl Into<DataVersion>, func: impl DataConverterFunc<$converted_type> + 'static) {
                let dyn_box: Box<dyn DataConverterFunc<$converted_type>> = Box::new(func);
                let converter = DataConverter::new(version, dyn_box);
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
            pub fn $method_name(&mut self, version: impl Into<DataVersion>, value: $element_type) {
                self.$field_name.entry(version.into()).or_default().push(value);
            }
        }
    }
}

type DynDataConverterFunc<T> = Box<dyn DataConverterFunc<T>>;

pub struct MapDataType<T: Types + ?Sized> {
    pub name: String,
    structure_converters: Vec<DataConverter<T::Map, DynDataConverterFunc<T::Map>>>,
    structure_walkers: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataWalker<T>>>>,
    structure_hooks: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataHook<T::Map>>>>,
}
structure_converters!(MapDataType, structure_converters, T::Map);
version_list!(MapDataType, add_structure_walker, structure_walkers, Box<dyn DataWalker<T>>);
version_list!(MapDataType, add_structure_hook, structure_hooks, Box<dyn DataHook<T::Map>>);
impl<T: Types + ?Sized> MapDataType<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
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
    converters: Vec<DataConverter<T::Object, DynDataConverterFunc<T::Object>>>,
    structure_hooks: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataHook<T::Object>>>>,
}
structure_converters!(ObjectDataType, converters, T::Object);
version_list!(ObjectDataType, add_structure_hook, structure_hooks, Box<dyn DataHook<T::Object>>);

impl<T: Types + ?Sized> ObjectDataType<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
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

type WalkersById<T> = Vec<Rc<dyn DataWalker<T>>>;

pub struct IdDataType<T: Types + ?Sized> {
    pub name: String,
    structure_converters: Vec<DataConverter<T::Map, DynDataConverterFunc<T::Map>>>,
    structure_walkers: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataWalker<T>>>>,
    structure_hooks: std::collections::BTreeMap<DataVersion, Vec<Box<dyn DataHook<T::Map>>>>,
    walkers_by_id: crate::Map<String, std::collections::BTreeMap<DataVersion, WalkersById<T>>>,
}
structure_converters!(IdDataType, structure_converters, T::Map);
version_list!(IdDataType, add_structure_walker, structure_walkers, Box<dyn DataWalker<T>>);
version_list!(IdDataType, add_structure_hook, structure_hooks, Box<dyn DataHook<T::Map>>);

impl<T: 'static + Types + ?Sized> IdDataType<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structure_converters: Vec::new(),
            structure_walkers: std::collections::BTreeMap::new(),
            structure_hooks: std::collections::BTreeMap::new(),
            walkers_by_id: crate::Map::default(),
        }
    }

    pub fn add_converter_for_id(&mut self, id: impl Into<String>, version: impl Into<DataVersion>, converter_func: impl DataConverterFunc<T::Map> + 'static) {
        let id_str = id.into();
        self.add_structure_converter(
            version,
            data_converter_func::<T::Map, _>(move |data, from_version, to_version| {
                    if data.get_string("id") == Some(&id_str) {
                        converter_func.convert(data, from_version, to_version);
                    }
                })
        );
    }

    pub fn add_walker_for_id(&mut self, version: impl Into<DataVersion>, id: impl Into<String>, walker: Rc<dyn DataWalker<T>>) {
        self.walkers_by_id.entry(id.into()).or_default().entry(version.into()).or_default().push(walker);
    }

    pub fn copy_walkers(&mut self, version: impl Into<DataVersion> + Clone, from_id: &str, to_id: impl Into<String> + Clone) {
        if let Some(from_versions) = self.walkers_by_id.get(from_id) {
            if let Some((_, from_walkers)) = from_versions.range(..=version.clone().into()).next_back() {
                for walker in from_walkers.clone() {
                    self.add_walker_for_id(version.clone().into(), to_id.clone().into(), walker);
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

pub fn data_walker<T, F>(func: F) -> impl DataWalker<T>
    where T: Types + ?Sized, F: Fn(&mut T::Map, DataVersion, DataVersion)
{
    struct DataWalkerImpl<F>(F);
    impl<T, F> DataWalker<T> for DataWalkerImpl<F>
        where T: Types + ?Sized, F: Fn(&mut T::Map, DataVersion, DataVersion)
    {
        fn walk(&self, data: &mut T::Map, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(data, from_version, to_version)
        }
    }
    DataWalkerImpl(func)
}
