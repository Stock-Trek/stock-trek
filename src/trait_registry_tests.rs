use crate::{
    register_algorithm,
    traitreg::{self, RegisteredImplWrapper, TraitRegStorage},
};

trait TestAlgorithm {
    fn value(&self) -> u8;
}

#[derive(Default)]
struct DefaultAlgorithm;

#[register_algorithm(default)]
impl TestAlgorithm for DefaultAlgorithm {
    fn value(&self) -> u8 {
        1
    }
}

struct NewAlgorithm;

impl NewAlgorithm {
    fn new() -> Self {
        Self
    }
}

#[register_algorithm(new)]
impl TestAlgorithm for NewAlgorithm {
    fn value(&self) -> u8 {
        2
    }
}

struct NoConstructorAlgorithm;

#[register_algorithm]
impl TestAlgorithm for NoConstructorAlgorithm {
    fn value(&self) -> u8 {
        3
    }
}

#[traitreg::registry(TestAlgorithm)]
static TEST_REGISTRY: () = ();

fn find<'a>(
    registry: &'a TraitRegStorage<Box<dyn TestAlgorithm>>,
    name: &str,
) -> &'a RegisteredImplWrapper<Box<dyn TestAlgorithm>> {
    registry
        .iter()
        .find(|registration| registration.name() == name)
        .expect("registration should exist")
}

#[test]
fn registers_every_implementation_with_metadata() {
    let mut names: Vec<_> = TEST_REGISTRY
        .iter()
        .map(|registration| registration.name())
        .collect();
    names.sort_unstable();

    assert_eq!(
        names,
        ["DefaultAlgorithm", "NewAlgorithm", "NoConstructorAlgorithm"]
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
fn instantiates_only_implementations_with_constructors() {
    let default = find(&TEST_REGISTRY, "DefaultAlgorithm");
    let new = find(&TEST_REGISTRY, "NewAlgorithm");
    let none = find(&TEST_REGISTRY, "NoConstructorAlgorithm");

    assert!(default.has_constructor());
    assert_eq!(
        default.instanciate().expect("default constructor").value(),
        1
    );
    assert!(new.has_constructor());
    assert_eq!(new.instanciate().expect("new constructor").value(), 2);
    assert!(!none.has_constructor());
    assert!(none.instanciate().is_none());

    let mut values: Vec<_> = TEST_REGISTRY
        .instanciate_all()
        .map(|algorithm| algorithm.value())
        .collect();
    values.sort_unstable();
    assert_eq!(values, [1, 2]);
}

#[test]
fn keeps_registries_for_different_traits_separate() {
    trait OtherAlgorithm {
        fn value(&self) -> u8;
    }

    #[derive(Default)]
    struct OtherAlgorithmImpl;

    #[register_algorithm(default)]
    impl OtherAlgorithm for OtherAlgorithmImpl {
        fn value(&self) -> u8 {
            9
        }
    }

    #[traitreg::registry(OtherAlgorithm)]
    static OTHER_REGISTRY: () = ();

    let test_registry = TraitRegStorage::<Box<dyn TestAlgorithm>>::new("TestAlgorithm");
    let other_registry = TraitRegStorage::<Box<dyn OtherAlgorithm>>::new("OtherAlgorithm");

    assert_eq!(test_registry.iter().count(), 3);
    assert_eq!(other_registry.iter().count(), 1);
    assert_eq!(
        other_registry
            .iter()
            .next()
            .expect("other registration")
            .instanciate()
            .expect("other constructor")
            .value(),
        9
    );
}
