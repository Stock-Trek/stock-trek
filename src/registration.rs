//! Runtime support for implementations annotated with `#[register_algorithm]`.
//!
//! The macro records metadata and a constructor for every annotated trait
//! implementation. Registrations are collected from the module's initialiser
//! array, which runs before the host calls into an algorithm. This also works
//! for WebAssembly, where constructors may run in a different order than on
//! native targets.
//!
//! Every implementation is constructed through [`Default`]. Use
//! [`RegisteredImpl::all`] to read the registrations for a trait.

use crate::algorithm::Algorithm;
use std::{
    any::{Any, TypeId},
    fmt::{self, Debug, Formatter},
    sync::{Mutex, MutexGuard, PoisonError},
};

type RegistrationEntries = Vec<(TypeId, Box<dyn Any + Send>)>;

static REGISTRATIONS: Mutex<RegistrationEntries> = Mutex::new(Vec::new());

/// Metadata and a constructor for a registered trait implementation.
///
/// Values are produced by `#[register_algorithm]`; read them with
/// [`RegisteredImpl::all`].
pub struct RegisteredImpl<Trait> {
    instantiate: fn() -> Trait,
    name: &'static str,
    path: &'static str,
    file: &'static str,
    module_path: &'static str,
    trait_name: &'static str,
}

impl<Trait> RegisteredImpl<Trait> {
    /// Creates the metadata stored for one annotated implementation.
    #[doc(hidden)]
    pub fn new(
        instantiate: fn() -> Trait,
        name: &'static str,
        path: &'static str,
        file: &'static str,
        module_path: &'static str,
        trait_name: &'static str,
    ) -> Self {
        Self {
            instantiate,
            name,
            path,
            file,
            module_path,
            trait_name,
        }
    }

    /// Constructs the implementation through [`Default`].
    pub fn instantiate(&self) -> Trait {
        (self.instantiate)()
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

impl<Trait: 'static> RegisteredImpl<Trait> {
    /// Returns every implementation registered for `Trait`.
    pub fn all() -> Vec<Self> {
        let trait_id = TypeId::of::<Trait>();
        Registration::entries()
            .iter()
            .filter(|(registered_trait_id, _)| *registered_trait_id == trait_id)
            .filter_map(|(_, registration)| registration.downcast_ref::<Self>())
            .cloned()
            .collect()
    }
}

impl<Trait> Clone for RegisteredImpl<Trait> {
    fn clone(&self) -> Self {
        Self {
            instantiate: self.instantiate,
            name: self.name,
            path: self.path,
            file: self.file,
            module_path: self.module_path,
            trait_name: self.trait_name,
        }
    }
}

impl<Trait> Debug for RegisteredImpl<Trait> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredImpl")
            .field("Type Name", &self.name)
            .field("Type Path", &self.path)
            .field("Trait Name", &self.trait_name)
            .field("Module Path", &self.module_path)
            .field("File", &self.file)
            .finish()
    }
}

/// Entry point used by generated registration constructors.
#[doc(hidden)]
#[derive(Debug)]
pub struct Registration;

impl Registration {
    fn entries() -> MutexGuard<'static, RegistrationEntries> {
        REGISTRATIONS.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Adds an implementation to the process-wide registration list.
    #[doc(hidden)]
    pub fn register<Trait: 'static>(registration: RegisteredImpl<Trait>) {
        Self::entries().push((TypeId::of::<Trait>(), Box::new(registration)));
    }
}

impl RegisteredImpl<Box<dyn Algorithm>> {
    /// Returns every implementation registered for [`Algorithm`].
    pub fn algorithms() -> Vec<Self> {
        Self::all()
    }
}
