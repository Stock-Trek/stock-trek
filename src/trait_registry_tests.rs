use crate::{
    register_algorithm,
    traitreg::{self, RegisteredImplWrapper, TraitRegStorage},
};

trait TestAlgorithm {
    fn value(&self) -> u8;
}

#[derive(Default)]
struct FirstAlgorithm;

#[register_algorithm]
impl TestAlgorithm for FirstAlgorithm {
    fn value(&self) -> u8 {
        1
    }
}

struct SecondAlgorithm {
    value: u8,
}

impl Default for SecondAlgorithm {
    fn default() -> Self {
        Self { value: 2 }
    }
}

#[register_algorithm]
impl TestAlgorithm for SecondAlgorithm {
    fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Default)]
struct ThirdAlgorithm;

#[register_algorithm]
impl TestAlgorithm for ThirdAlgorithm {
    fn value(&self) -> u8 {
        3
    }
}

trait OtherAlgorithm {
    fn value(&self) -> u8;
}

#[derive(Default)]
struct OtherAlgorithmImpl;

#[register_algorithm]
impl OtherAlgorithm for OtherAlgorithmImpl {
    fn value(&self) -> u8 {
        9
    }
}

#[traitreg::registry(TestAlgorithm)]
static TEST_REGISTRY: () = ();

#[traitreg::registry(OtherAlgorithm)]
static OTHER_REGISTRY: () = ();

struct RegistryTest;

impl RegistryTest {
    fn find<'a>(
        registry: &'a TraitRegStorage<Box<dyn TestAlgorithm>>,
        name: &str,
    ) -> &'a RegisteredImplWrapper<Box<dyn TestAlgorithm>> {
        registry
            .iter()
            .find(|registration| registration.name() == name)
            .expect("registration should exist")
    }

    fn registered_names() -> Vec<&'static str> {
        let mut names: Vec<_> = TEST_REGISTRY
            .iter()
            .map(|registration| registration.name())
            .collect();
        names.sort_unstable();
        names
    }
}

#[test]
fn registers_every_implementation_with_metadata() {
    assert_eq!(
        RegistryTest::registered_names(),
        ["FirstAlgorithm", "SecondAlgorithm", "ThirdAlgorithm"]
    );
    assert!(
        TEST_REGISTRY
            .iter()
            .all(|registration| registration.trait_name() == "TestAlgorithm")
    );
    assert!(
        TEST_REGISTRY
            .iter()
            .all(|registration| !registration.file().is_empty())
    );
    assert!(
        TEST_REGISTRY
            .iter()
            .all(|registration| !registration.module_path().is_empty())
    );
}

#[test]
fn instantiates_every_implementation_through_default() {
    let second = RegistryTest::find(&TEST_REGISTRY, "SecondAlgorithm");

    assert!(second.has_constructor());
    assert_eq!(
        second.instanciate().expect("default constructor").value(),
        2
    );

    let mut values: Vec<_> = TEST_REGISTRY
        .instanciate_all()
        .map(|algorithm| algorithm.value())
        .collect();
    values.sort_unstable();
    assert_eq!(values, [1, 2, 3]);
}

#[test]
fn keeps_registries_for_different_traits_separate() {
    assert_eq!(TEST_REGISTRY.iter().count(), 3);
    assert_eq!(OTHER_REGISTRY.iter().count(), 1);
    assert_eq!(
        OTHER_REGISTRY
            .iter()
            .next()
            .expect("other registration")
            .instanciate()
            .expect("other constructor")
            .value(),
        9
    );
}
