use crate::{JCompound, JValue, JValueMut};
use java_string::{JavaStr, JavaString};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::rc::Rc;

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

pub trait MapDataConverterFunc {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion);
}

pub fn map_data_converter_func<'a, F>(func: F) -> impl MapDataConverterFunc + 'a
where
    F: Fn(&mut JCompound, DataVersion, DataVersion) + 'a,
{
    struct DataConverterFuncImpl<F>(F);
    impl<F> MapDataConverterFunc for DataConverterFuncImpl<F>
    where
        F: Fn(&mut JCompound, DataVersion, DataVersion),
    {
        fn convert(
            &self,
            data: &mut JCompound,
            from_version: DataVersion,
            to_version: DataVersion,
        ) {
            (self.0)(data, from_version, to_version)
        }
    }
    DataConverterFuncImpl(func)
}

impl<T: MapDataConverterFunc + ?Sized> MapDataConverterFunc for &T {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<T: MapDataConverterFunc + ?Sized> MapDataConverterFunc for Box<T> {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

pub struct MapDataConverter<F: MapDataConverterFunc> {
    to_version: DataVersion,
    conversion_func: F,
}

impl<F: MapDataConverterFunc> MapDataConverter<F> {
    pub fn new(to_version: impl Into<DataVersion>, conversion_func: F) -> Self {
        Self {
            to_version: to_version.into(),
            conversion_func,
        }
    }

    #[inline]
    pub fn get_to_version(&self) -> DataVersion {
        self.to_version
    }

    pub fn convert(
        &self,
        data: &mut JCompound,
        from_version: impl Into<DataVersion>,
        to_version: impl Into<DataVersion>,
    ) {
        self.conversion_func
            .convert(data, from_version.into(), to_version.into())
    }
}

pub trait ValueDataConverterFunc {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion);
}

pub fn value_data_converter_func<'a, F>(func: F) -> impl ValueDataConverterFunc + 'a
where
    F: Fn(&mut JValueMut, DataVersion, DataVersion) + 'a,
{
    struct DataConverterFuncImpl<F>(F);
    impl<F> ValueDataConverterFunc for DataConverterFuncImpl<F>
    where
        F: Fn(&mut JValueMut, DataVersion, DataVersion),
    {
        fn convert(
            &self,
            data: &mut JValueMut,
            from_version: DataVersion,
            to_version: DataVersion,
        ) {
            (self.0)(data, from_version, to_version)
        }
    }
    DataConverterFuncImpl(func)
}

impl<T: ValueDataConverterFunc + ?Sized> ValueDataConverterFunc for &T {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<T: ValueDataConverterFunc + ?Sized> ValueDataConverterFunc for Box<T> {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

pub struct ValueDataConverter<F: ValueDataConverterFunc> {
    to_version: DataVersion,
    conversion_func: F,
}

impl<F: ValueDataConverterFunc> ValueDataConverter<F> {
    pub fn new(to_version: impl Into<DataVersion>, conversion_func: F) -> Self {
        Self {
            to_version: to_version.into(),
            conversion_func,
        }
    }

    #[inline]
    pub fn get_to_version(&self) -> DataVersion {
        self.to_version
    }

    pub fn convert(
        &self,
        data: &mut JValueMut,
        from_version: impl Into<DataVersion>,
        to_version: impl Into<DataVersion>,
    ) {
        self.conversion_func
            .convert(data, from_version.into(), to_version.into())
    }
}

pub trait DynamicDataConverterFunc {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion);
}

pub fn dynamic_data_converter_func<'a, F>(func: F) -> impl DynamicDataConverterFunc + 'a
where
    F: Fn(&mut JValue, DataVersion, DataVersion) + 'a,
{
    struct DataConverterFuncImpl<F>(F);
    impl<F> DynamicDataConverterFunc for DataConverterFuncImpl<F>
    where
        F: Fn(&mut JValue, DataVersion, DataVersion),
    {
        fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(data, from_version, to_version)
        }
    }
    DataConverterFuncImpl(func)
}

impl<T: DynamicDataConverterFunc + ?Sized> DynamicDataConverterFunc for &T {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<T: DynamicDataConverterFunc + ?Sized> DynamicDataConverterFunc for Box<T> {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

pub struct DynamicDataConverter<F: DynamicDataConverterFunc> {
    to_version: DataVersion,
    conversion_func: F,
}

impl<F: DynamicDataConverterFunc> DynamicDataConverter<F> {
    pub fn new(to_version: impl Into<DataVersion>, conversion_func: F) -> Self {
        Self {
            to_version: to_version.into(),
            conversion_func,
        }
    }

    #[inline]
    pub fn get_to_version(&self) -> DataVersion {
        self.to_version
    }

    pub fn convert(
        &self,
        data: &mut JValue,
        from_version: impl Into<DataVersion>,
        to_version: impl Into<DataVersion>,
    ) {
        self.conversion_func
            .convert(data, from_version.into(), to_version.into())
    }
}

macro_rules! impl_traits {
    ($converter:ident, $converter_func_trait:ident) => {
        impl<F: $converter_func_trait> core::fmt::Debug for $converter<F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    concat!(stringify!($converter), "{{{:?}}}"),
                    self.to_version
                )
            }
        }

        impl<F: $converter_func_trait> PartialEq for $converter<F> {
            fn eq(&self, other: &Self) -> bool {
                self.to_version == other.to_version
            }
        }

        impl<F: $converter_func_trait> Eq for $converter<F> {}

        impl<F: $converter_func_trait> PartialOrd for $converter<F> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<F: $converter_func_trait> Ord for $converter<F> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.to_version.cmp(&other.to_version)
            }
        }
    };
}

impl_traits!(MapDataConverter, MapDataConverterFunc);
impl_traits!(ValueDataConverter, ValueDataConverterFunc);
impl_traits!(DynamicDataConverter, DynamicDataConverterFunc);

pub struct ConversionError {
    pub message: String,
}

pub type Result<T> = core::result::Result<T, ConversionError>;

pub trait AbstractMapDataType {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion);
}

impl<T: AbstractMapDataType> AbstractMapDataType for &T {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<T: AbstractMapDataType> AbstractMapDataType for std::sync::RwLock<T> {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        let this = self.read().unwrap();
        T::convert(&*this, data, from_version, to_version)
    }
}

pub trait AbstractValueDataType {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion);
}

impl<T: AbstractValueDataType> AbstractValueDataType for &T {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<T: AbstractValueDataType> AbstractValueDataType for std::sync::RwLock<T> {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion) {
        let this = self.read().unwrap();
        T::convert(&*this, data, from_version, to_version)
    }
}

pub trait AbstractDynamicDataType {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion);
}

impl<T: AbstractDynamicDataType> AbstractDynamicDataType for &T {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, data, from_version, to_version)
    }
}

impl<T: AbstractDynamicDataType> AbstractDynamicDataType for std::sync::RwLock<T> {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
        let this = self.read().unwrap();
        T::convert(&*this, data, from_version, to_version)
    }
}

macro_rules! structure_converters {
    ($ty:ident, $field_name:ident, $data_converter:ident, $converter_func:ident) => {
        impl<'a> $ty<'a> {
            pub fn add_structure_converter(
                &mut self,
                version: impl Into<DataVersion>,
                func: impl $converter_func + 'a,
            ) {
                let dyn_box: Box<dyn $converter_func> = Box::new(func);
                let converter = $data_converter::new(version, dyn_box);
                let index = self.$field_name.binary_search(&converter);
                let index = match index {
                    Ok(i) => i,
                    Err(i) => i,
                };
                self.$field_name.insert(index, converter);
            }
        }
    };
}

macro_rules! version_list {
    ($ty:ident, $method_name:ident, $field_name:ident, $element_type:ty) => {
        impl<'a> $ty<'a> {
            pub fn $method_name(&mut self, version: impl Into<DataVersion>, value: $element_type) {
                self.$field_name
                    .entry(version.into())
                    .or_default()
                    .push(Box::new(value));
            }
        }
    };
}

type DynMapDataConverterFunc<'a> = Box<dyn MapDataConverterFunc + 'a>;

pub struct MapDataType<'a> {
    pub name: String,
    structure_converters: Vec<MapDataConverter<DynMapDataConverterFunc<'a>>>,
    structure_walkers: BTreeMap<DataVersion, Vec<Box<dyn MapDataWalker + 'a>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn MapDataHook + 'a>>>,
}
structure_converters!(
    MapDataType,
    structure_converters,
    MapDataConverter,
    MapDataConverterFunc
);
version_list!(
    MapDataType,
    add_structure_walker,
    structure_walkers,
    impl MapDataWalker + 'a
);
version_list!(
    MapDataType,
    add_structure_hook,
    structure_hooks,
    impl MapDataHook + 'a
);
impl<'a> MapDataType<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structure_converters: Vec::new(),
            structure_walkers: BTreeMap::new(),
            structure_hooks: BTreeMap::new(),
        }
    }
}

impl<'a> AbstractMapDataType for MapDataType<'a> {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.structure_converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self
                .structure_hooks
                .range(..=converter.get_to_version())
                .next_back();
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

type DynValueDataConverterFunc<'a> = Box<dyn ValueDataConverterFunc + 'a>;

pub struct ObjectDataType<'a> {
    pub name: String,
    converters: Vec<ValueDataConverter<DynValueDataConverterFunc<'a>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn ValueDataHook + 'a>>>,
}
structure_converters!(
    ObjectDataType,
    converters,
    ValueDataConverter,
    ValueDataConverterFunc
);
version_list!(
    ObjectDataType,
    add_structure_hook,
    structure_hooks,
    impl ValueDataHook + 'a
);

impl<'a> ObjectDataType<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            converters: Vec::new(),
            structure_hooks: BTreeMap::new(),
        }
    }
}

impl<'a> AbstractValueDataType for ObjectDataType<'a> {
    fn convert(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self
                .structure_hooks
                .range(..=converter.get_to_version())
                .next_back();
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

type DynDynamicDataConverterFunc<'a> = Box<dyn DynamicDataConverterFunc + 'a>;

pub struct DynamicDataType<'a> {
    pub name: String,
    structure_converters: Vec<DynamicDataConverter<DynDynamicDataConverterFunc<'a>>>,
    structure_walkers: BTreeMap<DataVersion, Vec<Box<dyn DynamicDataWalker + 'a>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn DynamicDataHook + 'a>>>,
}
structure_converters!(
    DynamicDataType,
    structure_converters,
    DynamicDataConverter,
    DynamicDataConverterFunc
);
version_list!(
    DynamicDataType,
    add_structure_walker,
    structure_walkers,
    impl DynamicDataWalker + 'a
);
version_list!(
    DynamicDataType,
    add_structure_hook,
    structure_hooks,
    impl DynamicDataHook + 'a
);

impl<'a> DynamicDataType<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structure_converters: Vec::new(),
            structure_walkers: BTreeMap::new(),
            structure_hooks: BTreeMap::new(),
        }
    }
}

impl<'a> AbstractDynamicDataType for DynamicDataType<'a> {
    fn convert(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.structure_converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self
                .structure_hooks
                .range(..=converter.get_to_version())
                .next_back();
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

type WalkersById<'a> = Vec<Rc<dyn MapDataWalker + 'a>>;

pub struct IdDataType<'a> {
    pub name: String,
    structure_converters: Vec<MapDataConverter<DynMapDataConverterFunc<'a>>>,
    structure_walkers: BTreeMap<DataVersion, Vec<Box<dyn MapDataWalker + 'a>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn MapDataHook + 'a>>>,
    walkers_by_id: BTreeMap<JavaString, BTreeMap<DataVersion, WalkersById<'a>>>,
}
structure_converters!(
    IdDataType,
    structure_converters,
    MapDataConverter,
    MapDataConverterFunc
);
version_list!(
    IdDataType,
    add_structure_walker,
    structure_walkers,
    impl MapDataWalker + 'a
);
version_list!(
    IdDataType,
    add_structure_hook,
    structure_hooks,
    impl MapDataHook + 'a
);

impl<'a> IdDataType<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structure_converters: Vec::new(),
            structure_walkers: BTreeMap::new(),
            structure_hooks: BTreeMap::new(),
            walkers_by_id: BTreeMap::new(),
        }
    }

    pub fn add_converter_for_id(
        &mut self,
        id: impl Into<JavaString>,
        version: impl Into<DataVersion>,
        converter_func: impl MapDataConverterFunc + 'a,
    ) {
        let id_str = id.into();
        self.add_structure_converter(
            version,
            map_data_converter_func(move |data, from_version, to_version| {
                if matches!(data.get("id"), Some(valence_nbt::Value::String(str)) if str == &id_str)
                {
                    converter_func.convert(data, from_version, to_version);
                }
            }),
        );
    }

    pub fn add_walker_for_id(
        &mut self,
        version: impl Into<DataVersion>,
        id: impl Into<JavaString>,
        walker: impl MapDataWalker + 'a,
    ) {
        self.walkers_by_id
            .entry(id.into())
            .or_default()
            .entry(version.into())
            .or_default()
            .push(Rc::new(walker));
    }

    pub fn copy_walkers(
        &mut self,
        version: impl Into<DataVersion> + Clone,
        from_id: impl AsRef<JavaStr>,
        to_id: impl Into<JavaString> + Clone,
    ) {
        if let Some(from_versions) = self.walkers_by_id.get(from_id.as_ref()) {
            if let Some((_, from_walkers)) =
                from_versions.range(..=version.clone().into()).next_back()
            {
                for walker in from_walkers.clone() {
                    self.walkers_by_id
                        .entry(to_id.clone().into())
                        .or_default()
                        .entry(version.clone().into())
                        .or_default()
                        .push(walker);
                }
            }
        }
    }
}

impl<'a> AbstractMapDataType for IdDataType<'a> {
    fn convert(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        for converter in &self.structure_converters {
            if converter.get_to_version() <= from_version {
                continue;
            }
            if converter.get_to_version() > to_version {
                break;
            }

            let hooks = self
                .structure_hooks
                .range(..=converter.get_to_version())
                .next_back();
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

        if let Some(valence_nbt::Value::String(id)) = data.get("id") {
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

pub trait MapDataHook {
    fn pre_hook(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion);
    fn post_hook(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion);
}

pub trait ValueDataHook {
    fn pre_hook(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion);
    fn post_hook(&self, data: &mut JValueMut, from_version: DataVersion, to_version: DataVersion);
}

pub trait DynamicDataHook {
    fn pre_hook(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion);
    fn post_hook(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion);
}

pub trait MapDataWalker {
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion);
}

pub fn map_data_walker<'a, F>(func: F) -> impl MapDataWalker + 'a
where
    F: Fn(&mut JCompound, DataVersion, DataVersion) + 'a,
{
    struct MapDataWalkerImpl<F>(F);
    impl<F> MapDataWalker for MapDataWalkerImpl<F>
    where
        F: Fn(&mut JCompound, DataVersion, DataVersion),
    {
        fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(data, from_version, to_version)
        }
    }
    MapDataWalkerImpl(func)
}

pub trait DynamicDataWalker {
    fn walk(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion);
}

pub fn dynamic_data_walker<'a, F>(func: F) -> impl DynamicDataWalker + 'a
where
    F: Fn(&mut JValue, DataVersion, DataVersion) + 'a,
{
    struct DynamicDataWalkerImpl<F>(F);
    impl<F> DynamicDataWalker for DynamicDataWalkerImpl<F>
    where
        F: Fn(&mut JValue, DataVersion, DataVersion),
    {
        fn walk(&self, data: &mut JValue, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(data, from_version, to_version)
        }
    }
    DynamicDataWalkerImpl(func)
}
