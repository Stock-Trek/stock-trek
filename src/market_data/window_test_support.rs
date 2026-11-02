use super::{aligned_window::AlignedWindow, rolling_window::RollingWindow};
use chrono::{DateTime, Duration, Months, Utc};

/// Test-only arithmetic helpers for the window enums.
pub(crate) trait CheckedWindow {
    /// Adds the window length to a UTC date and time.
    fn checked_add(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>>;

    /// Subtracts the window length from a UTC date and time.
    fn checked_sub(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>>;
}

impl CheckedWindow for RollingWindow {
    fn checked_add(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let millis = i64::try_from(self.millis_exact()?).ok()?;
        date_time.checked_add_signed(Duration::milliseconds(millis))
    }

    fn checked_sub(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let millis = i64::try_from(self.millis_exact()?).ok()?;
        date_time.checked_sub_signed(Duration::milliseconds(millis))
    }
}

impl CheckedWindow for AlignedWindow {
    fn checked_add(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        if let Some(millis) = self.millis_exact() {
            let millis = i64::try_from(millis).ok()?;
            date_time.checked_add_signed(Duration::milliseconds(millis))
        } else {
            date_time.checked_add_months(Months::new(self.months_exact()?))
        }
    }

    fn checked_sub(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        if let Some(millis) = self.millis_exact() {
            let millis = i64::try_from(millis).ok()?;
            date_time.checked_sub_signed(Duration::milliseconds(millis))
        } else {
            date_time.checked_sub_months(Months::new(self.months_exact()?))
        }
    }
}

/// Defines the behaviour shared by the window enums.
///
/// Calendar windows are excluded from the fixed-length assertions because their
/// length varies with the calendar.
macro_rules! window_tests {
    ($window:ty, $($variant:expr => $label:literal),+ $(,)?) => {
        #[test]
        fn display_uses_short_labels() {
            $(
                assert_eq!($variant.to_string(), $label);
            )+
        }

        #[test]
        fn display_round_trips_through_try_from() {
            for window in <$window as ::strum::IntoEnumIterator>::iter() {
                assert_eq!(window.label(), window.to_string());
                assert_eq!(
                    <$window>::try_from(window.to_string().as_str()).unwrap(),
                    window
                );
                assert_eq!(window.to_string().parse::<$window>().unwrap(), window);
            }
        }

        #[test]
        fn serde_round_trips_short_labels() {
            for window in <$window as ::strum::IntoEnumIterator>::iter() {
                let json = serde_json::to_string(&window).unwrap();

                assert_eq!(json, format!("\"{}\"", window.label()));
                assert_eq!(
                    serde_json::from_str::<$window>(&json).unwrap(),
                    window
                );
            }
        }

        #[test]
        fn serde_rejects_names() {
            assert!(serde_json::from_str::<$window>("\"seconds1\"").is_err());
        }

        #[test]
        fn checked_add_and_sub_use_the_window_length() {
            let date_time =
                chrono::DateTime::from_timestamp_millis(1_700_000_000_123).unwrap();

            for window in
                <$window as ::strum::IntoEnumIterator>::iter().filter(|window| window.millis_exact().is_some())
            {
                let added = window.checked_add(date_time).unwrap();
                let subtracted = window.checked_sub(date_time).unwrap();
                let forward = added.timestamp_millis() - date_time.timestamp_millis();
                let backward = date_time.timestamp_millis() - subtracted.timestamp_millis();

                assert_eq!(forward, backward, "{window} is not symmetric");
                assert_eq!(
                    i64::try_from(window.millis_exact().unwrap()).unwrap(),
                    forward,
                    "{window} does not advance by its own length"
                );
            }
        }

        #[test]
        fn checked_add_and_sub_return_none_when_out_of_range() {
            for window in <$window as ::strum::IntoEnumIterator>::iter() {
                assert_eq!(
                    window.checked_add(chrono::DateTime::<chrono::Utc>::MAX_UTC),
                    None,
                    "{window} should not add past the maximum date"
                );
                assert_eq!(
                    window.checked_sub(chrono::DateTime::<chrono::Utc>::MIN_UTC),
                    None,
                    "{window} should not subtract past the minimum date"
                );
            }
        }
    };
}

pub(crate) use window_tests;
