//! Trait registry runtime used by the registration macros.
//!
//! Implementations are registered with `#[register_algorithm]` and collected
//! lazily by [`registry`]. Every implementation is constructed through
//! [`Default`] when the registry is read. Lazy collection makes the order in
//! which constructors run irrelevant, including in WebAssembly where all
//! constructors run from the module's `.init_array` before the host calls into
//! the algorithm.

pub use stock_trek_macros::registry;

use std::{
    any::Any,
    fmt::{self, Debug, Formatter},
    sync::Mutex,
};

/// All registrations across every trait, keyed by the trait's identifier.
static REGISTRATIONS: Mutex<Vec<(&'static str, Box<dyn Any + Send>)>> = Mutex::new(Vec::new());

/// Metadata and constructor for a registered trait implementation.
///
/// Implemented by `#[register_algorithm]` for each annotated implementation.
/// The implementing type must implement [`Default`].
pub trait RegisteredImpl<Trait> {
    /// Constructs the implementation through [`Default`].
    const INSTANCIATE: fn() -> Option<Trait>;
    /// Whether the implementation has a constructor.
    const HAS_CONSTRUCTOR: bool;
    /// The implementing type's name.
    const NAME: &'static str;
    /// The implementing type's path.
    const PATH: &'static str;
    /// The file containing the implementation.
    const FILE: &'static str;
    /// The module containing the implementation.
    const MODULE_PATH: &'static str;
    /// The trait's name.
    const TRAIT_NAME: &'static str;
}

/// Entry point used by generated registration constructors.
#[doc(hidden)]
#[derive(Debug)]
pub struct Registration;

impl Registration {
    /// Adds an implementation to the process-wide registration list.
    pub fn register<Trait: 'static, Type: RegisteredImpl<Trait>>() {
        let registration = RegisteredImplWrapper::<Trait> {
            instanciate: Type::INSTANCIATE,
            has_constructor: Type::HAS_CONSTRUCTOR,
            name: Type::NAME,
            path: Type::PATH,
            file: Type::FILE,
            module_path: Type::MODULE_PATH,
            trait_name: Type::TRAIT_NAME,
        };
        let mut registrations = REGISTRATIONS.lock().expect("Trait registry mutex poisoned");
        registrations.push((Type::TRAIT_NAME, Box::new(registration)));
    }
}

/// A snapshot of the implementations registered for one trait.
pub struct TraitRegStorage<Trait> {
    impls: Vec<RegisteredImplWrapper<Trait>>,
}

impl<Trait: 'static> TraitRegStorage<Trait> {
    /// Collects all implementations registered under `trait_name`.
    pub fn new(trait_name: &'static str) -> Self {
        let registrations = REGISTRATIONS.lock().expect("Trait registry mutex poisoned");
        let impls = registrations
            .iter()
            .filter(|(name, _)| *name == trait_name)
            .filter_map(|(_, registration)| {
                registration.downcast_ref::<RegisteredImplWrapper<Trait>>()
            })
            .cloned()
            .collect();
        Self { impls }
    }

    /// Collects all implementations registered under `trait_name`.
    #[doc(hidden)]
    pub fn __new(trait_name: &'static str) -> Self {
        Self::new(trait_name)
    }

    /// Iterates over the registered implementations.
    pub fn iter(&self) -> core::slice::Iter<'_, RegisteredImplWrapper<Trait>> {
        self.impls.iter()
    }

    /// Constructs every implementation.
    pub fn instanciate_all(&self) -> impl Iterator<Item = Trait> + '_ {
        self.impls.iter().filter_map(|item| item.instanciate())
    }
}

impl<Trait> Debug for TraitRegStorage<Trait> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TraitRegStorage")
            .field("impls", &self.impls)
            .finish()
    }
}

/// A registered implementation and its metadata.
pub struct RegisteredImplWrapper<Trait> {
    instanciate: fn() -> Option<Trait>,
    has_constructor: bool,
    name: &'static str,
    path: &'static str,
    file: &'static str,
    module_path: &'static str,
    trait_name: &'static str,
}

impl<Trait> RegisteredImplWrapper<Trait> {
    /// Constructs the implementation through [`Default`].
    pub fn instanciate(&self) -> Option<Trait> {
        (self.instanciate)()
    }

    /// Whether the implementation has a constructor.
    pub fn has_constructor(&self) -> bool {
        self.has_constructor
    }

    /// The implementing type's name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The implementing type's path.
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// The file containing the implementation.
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// The module containing the implementation.
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The trait's name.
    pub fn trait_name(&self) -> &'static str {
        self.trait_name
    }
}

impl<Trait> Clone for RegisteredImplWrapper<Trait> {
    fn clone(&self) -> Self {
        Self {
            instanciate: self.instanciate,
            has_constructor: self.has_constructor,
            name: self.name,
            path: self.path,
            file: self.file,
            module_path: self.module_path,
            trait_name: self.trait_name,
        }
    }
}

impl<Trait> Debug for RegisteredImplWrapper<Trait> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredImpl")
            .field("Type Name", &self.name)
            .field("Type Path", &self.path)
            .field("Trait Name", &self.trait_name)
            .field("Has Constructor", &self.has_constructor)
            .field("Module Path", &self.module_path)
            .field("File", &self.file)
            .finish()
    }
}
