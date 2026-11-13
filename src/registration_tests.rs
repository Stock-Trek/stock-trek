use crate::{RegisteredImpl, register_algorithm};

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

struct RegistrationTest;

impl RegistrationTest {
    fn test_registrations() -> Vec<RegisteredImpl<Box<dyn TestAlgorithm>>> {
        RegisteredImpl::<Box<dyn TestAlgorithm>>::all()
    }

    fn registered_names() -> Vec<&'static str> {
        let mut names: Vec<_> = Self::test_registrations()
            .iter()
            .map(|registration| registration.name())
            .collect();
        names.sort_unstable();
        names
    }

    fn values() -> Vec<u8> {
        let mut values: Vec<_> = Self::test_registrations()
            .iter()
            .map(|registration| registration.instantiate().value())
            .collect();
        values.sort_unstable();
        values
    }
}

#[test]
fn registers_every_implementation_with_metadata() {
    let registrations = RegistrationTest::test_registrations();

    assert_eq!(
        RegistrationTest::registered_names(),
        ["FirstAlgorithm", "SecondAlgorithm", "ThirdAlgorithm"]
    );
    assert!(
        registrations
            .iter()
            .all(|registration| registration.trait_name() == "TestAlgorithm")
    );
    assert!(
        registrations
            .iter()
            .all(|registration| !registration.file().is_empty())
    );
    assert!(
        registrations
            .iter()
            .all(|registration| !registration.module_path().is_empty())
    );
}

#[test]
fn instantiates_every_implementation_through_default() {
    assert_eq!(RegistrationTest::values(), [1, 2, 3]);
}

#[test]
fn keeps_registrations_for_different_traits_separate() {
    let other = RegisteredImpl::<Box<dyn OtherAlgorithm>>::all();

    assert_eq!(RegistrationTest::test_registrations().len(), 3);
    assert_eq!(other.len(), 1);
    assert_eq!(other[0].instantiate().value(), 9);
}
