use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;
use valence_nbt::Compound;

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
    type Context;

    fn convert(&self, context: Self::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion);
}

pub fn map_data_converter_func<F, C>(func: F) -> impl MapDataConverterFunc<Context = C>
where
    F: Fn(C, &mut Compound, DataVersion, DataVersion),
{
    struct DataConverterFuncImpl<F, C>(F, PhantomData<C>);
    impl<F, C> MapDataConverterFunc for DataConverterFuncImpl<F, C>
    where
        F: Fn(C, &mut Compound, DataVersion, DataVersion),
    {
        type Context = C;

        fn convert(&self, context: C, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(context, data, from_version, to_version)
        }
    }
    DataConverterFuncImpl(func, PhantomData)
}

impl<T: MapDataConverterFunc + ?Sized> MapDataConverterFunc for &T {
    type Context = T::Context;

    fn convert(&self, context: T::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, context, data, from_version, to_version)
    }
}

impl<T: MapDataConverterFunc + ?Sized> MapDataConverterFunc for Box<T> {
    type Context = T::Context;

    fn convert(&self, context: T::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(&*self, context, data, from_version, to_version)
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
        context: F::Context,
        data: &mut Compound,
        from_version: impl Into<DataVersion>,
        to_version: impl Into<DataVersion>,
    ) {
        self.conversion_func
            .convert(context, data, from_version.into(), to_version.into())
    }
}

pub trait ValueDataConverterFunc {
    type Context;

    fn convert(
        &self,
        context: Self::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    );
}

pub fn value_data_converter_func<F, C>(func: F) -> impl ValueDataConverterFunc<Context = C>
where
    F: Fn(C, &mut valence_nbt::value::ValueMut, DataVersion, DataVersion),
{
    struct DataConverterFuncImpl<F, C>(F, PhantomData<C>);
    impl<F, C> ValueDataConverterFunc for DataConverterFuncImpl<F, C>
    where
        F: Fn(C, &mut valence_nbt::value::ValueMut, DataVersion, DataVersion),
    {
        type Context = C;

        fn convert(
            &self,
            context: C,
            data: &mut valence_nbt::value::ValueMut,
            from_version: DataVersion,
            to_version: DataVersion,
        ) {
            (self.0)(context, data, from_version, to_version)
        }
    }
    DataConverterFuncImpl(func, PhantomData)
}

impl<T: ValueDataConverterFunc + ?Sized> ValueDataConverterFunc for &T {
    type Context = T::Context;

    fn convert(
        &self,
        context: T::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    ) {
        T::convert(self, context, data, from_version, to_version)
    }
}

impl<T: ValueDataConverterFunc + ?Sized> ValueDataConverterFunc for Box<T> {
    type Context = T::Context;

    fn convert(
        &self,
        context: T::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    ) {
        T::convert(&*self, context, data, from_version, to_version)
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
        context: F::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: impl Into<DataVersion>,
        to_version: impl Into<DataVersion>,
    ) {
        self.conversion_func
            .convert(context, data, from_version.into(), to_version.into())
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
                self.to_version.partial_cmp(&other.to_version)
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

pub struct ConversionError {
    pub message: String,
}

pub type Result<T> = core::result::Result<T, ConversionError>;

pub trait AbstractMapDataType {
    type Context;

    fn convert(&self, context: Self::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion);
}

impl<T: AbstractMapDataType> AbstractMapDataType for &T {
    type Context = T::Context;

    fn convert(&self, context: T::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(self, context, data, from_version, to_version)
    }
}

impl<T: AbstractMapDataType> AbstractMapDataType for core::cell::RefCell<T> {
    type Context = T::Context;

    fn convert(&self, context: T::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        T::convert(&*self.borrow(), context, data, from_version, to_version)
    }
}

pub trait AbstractValueDataType {
    type Context;

    fn convert(
        &self,
        context: Self::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    );
}

impl<T: AbstractValueDataType> AbstractValueDataType for &T {
    type Context = T::Context;

    fn convert(
        &self,
        context: T::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    ) {
        T::convert(self, context, data, from_version, to_version)
    }
}

impl<T: AbstractValueDataType> AbstractValueDataType for core::cell::RefCell<T> {
    type Context = T::Context;

    fn convert(
        &self,
        context: T::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    ) {
        T::convert(&*self.borrow(), context, data, from_version, to_version)
    }
}

macro_rules! structure_converters {
    ($ty:ident, $field_name:ident, $data_converter:ident, $converter_func:ident) => {
        impl<C> $ty<C> {
            pub fn add_structure_converter(
                &mut self,
                version: impl Into<DataVersion>,
                func: impl $converter_func<Context = C> + 'static,
            ) {
                let dyn_box: Box<dyn $converter_func<Context = C>> = Box::new(func);
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
        impl<C> $ty<C> {
            pub fn $method_name(&mut self, version: impl Into<DataVersion>, value: $element_type) {
                self.$field_name
                    .entry(version.into())
                    .or_default()
                    .push(Box::new(value));
            }
        }
    };
}

type DynMapDataConverterFunc<C> = Box<dyn MapDataConverterFunc<Context = C>>;

pub struct MapDataType<C> {
    pub name: String,
    structure_converters: Vec<MapDataConverter<DynMapDataConverterFunc<C>>>,
    structure_walkers: BTreeMap<DataVersion, Vec<Box<dyn DataWalker<Context = C>>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn MapDataHook<Context = C>>>>,
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
    impl DataWalker<Context = C> + 'static
);
version_list!(
    MapDataType,
    add_structure_hook,
    structure_hooks,
    impl MapDataHook<Context = C> + 'static
);
impl<C> MapDataType<C> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structure_converters: Vec::new(),
            structure_walkers: BTreeMap::new(),
            structure_hooks: BTreeMap::new(),
        }
    }
}

impl<C> AbstractMapDataType for MapDataType<C> where C: Copy {
    type Context = C;

    fn convert(&self, context: C, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
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
                    hook.pre_hook(context, data, from_version, to_version);
                }
            }

            converter.convert(context, data, from_version, to_version);

            // possibly new data format, update hooks
            let hooks = self.structure_hooks.range(..=to_version).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks.iter().rev() {
                    hook.post_hook(context, data, from_version, to_version);
                }
            }
        }

        let hooks = self.structure_hooks.range(..=to_version).next_back();
        if let Some((_, hooks)) = hooks {
            for hook in hooks {
                hook.pre_hook(context, data, from_version, to_version);
            }
        }

        let walkers = self.structure_walkers.range(..=to_version).next_back();
        if let Some((_, walkers)) = walkers {
            for walker in walkers {
                walker.walk(context, data, from_version, to_version);
            }
        }

        if let Some((_, hooks)) = hooks {
            for hook in hooks.iter().rev() {
                hook.post_hook(context, data, from_version, to_version);
            }
        }
    }
}

type DynValueDataConverterFunc<C> = Box<dyn ValueDataConverterFunc<Context = C>>;

pub struct ObjectDataType<C> {
    pub name: String,
    converters: Vec<ValueDataConverter<DynValueDataConverterFunc<C>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn ValueDataHook<Context = C>>>>,
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
    impl ValueDataHook<Context = C> + 'static
);

impl<C> ObjectDataType<C> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            converters: Vec::new(),
            structure_hooks: BTreeMap::new(),
        }
    }
}

impl<C> AbstractValueDataType for ObjectDataType<C> where C: Copy {
    type Context = C;

    fn convert(
        &self,
        context: C,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    ) {
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
                    hook.pre_hook(context, data, from_version, to_version);
                }
            }

            converter.convert(context, data, from_version, to_version);

            // possibly new data format, update hooks
            let hooks = self.structure_hooks.range(..=to_version).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks.iter().rev() {
                    hook.post_hook(context, data, from_version, to_version);
                }
            }
        }
    }
}

type WalkersById<C> = Vec<Rc<dyn DataWalker<Context = C>>>;

pub struct IdDataType<C> {
    pub name: String,
    structure_converters: Vec<MapDataConverter<DynMapDataConverterFunc<C>>>,
    structure_walkers: BTreeMap<DataVersion, Vec<Box<dyn DataWalker<Context = C>>>>,
    structure_hooks: BTreeMap<DataVersion, Vec<Box<dyn MapDataHook<Context = C>>>>,
    walkers_by_id: BTreeMap<String, BTreeMap<DataVersion, WalkersById<C>>>,
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
    impl DataWalker<Context = C> + 'static
);
version_list!(
    IdDataType,
    add_structure_hook,
    structure_hooks,
    impl MapDataHook<Context = C> + 'static
);

impl<C> IdDataType<C> {
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
        id: impl Into<String>,
        version: impl Into<DataVersion>,
        converter_func: impl MapDataConverterFunc<Context = C> + 'static,
    ) {
        let id_str = id.into();
        self.add_structure_converter(
            version,
            map_data_converter_func(move |context, data, from_version, to_version| {
                if matches!(data.get("id"), Some(valence_nbt::Value::String(str)) if str == &id_str)
                {
                    converter_func.convert(context, data, from_version, to_version);
                }
            }),
        );
    }

    pub fn add_walker_for_id(
        &mut self,
        version: impl Into<DataVersion>,
        id: impl Into<String>,
        walker: impl DataWalker<Context = C> + 'static,
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
        from_id: &str,
        to_id: impl Into<String> + Clone,
    ) {
        if let Some(from_versions) = self.walkers_by_id.get(from_id) {
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

impl<C> AbstractMapDataType for IdDataType<C> where C: Copy {
    type Context = C;

    fn convert(&self, context: C, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
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
                    hook.pre_hook(context, data, from_version, to_version);
                }
            }

            converter.convert(context, data, from_version, to_version);

            // possibly new data format, update hooks
            let hooks = self.structure_hooks.range(..=to_version).next_back();
            if let Some((_, hooks)) = hooks {
                for hook in hooks {
                    hook.post_hook(context, data, from_version, to_version);
                }
            }
        }

        // run pre hooks

        let hooks = self.structure_hooks.range(..=to_version).next_back();
        if let Some((_, hooks)) = hooks {
            for hook in hooks.iter().rev() {
                hook.pre_hook(context, data, from_version, to_version);
            }
        }

        // run all walkers

        let walkers = self.structure_walkers.range(..=to_version).next_back();
        if let Some((_, walkers)) = walkers {
            for walker in walkers {
                walker.walk(context, data, from_version, to_version);
            }
        }

        if let Some(valence_nbt::Value::String(id)) = data.get("id") {
            if let Some(walkers_by_version) = self.walkers_by_id.get(id) {
                if let Some((_, walkers)) = walkers_by_version.range(..=to_version).next_back() {
                    for walker in walkers {
                        walker.walk(context, data, from_version, to_version);
                    }
                }
            }
        }

        // run post hooks

        if let Some((_, hooks)) = hooks {
            for hook in hooks.iter().rev() {
                hook.post_hook(context, data, from_version, to_version);
            }
        }
    }
}

pub trait MapDataHook {
    type Context;

    fn pre_hook(&self, context: Self::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion);
    fn post_hook(&self, context: Self::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion);
}

pub trait ValueDataHook {
    type Context;

    fn pre_hook(
        &self,
        context: Self::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    );
    fn post_hook(
        &self,
        context: Self::Context,
        data: &mut valence_nbt::value::ValueMut,
        from_version: DataVersion,
        to_version: DataVersion,
    );
}

pub trait DataWalker {
    type Context;

    fn walk(&self, context: Self::Context, data: &mut Compound, from_version: DataVersion, to_version: DataVersion);
}

pub fn data_walker<F, C>(func: F) -> impl DataWalker<Context = C>
where
    F: Fn(C, &mut Compound, DataVersion, DataVersion),
{
    struct DataWalkerImpl<F, C>(F, PhantomData<C>);
    impl<F, C> DataWalker for DataWalkerImpl<F, C>
    where
        F: Fn(C, &mut Compound, DataVersion, DataVersion),
    {
        type Context = C;

        fn walk(&self, context: C, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
            (self.0)(context, data, from_version, to_version)
        }
    }
    DataWalkerImpl(func, PhantomData)
}
