use super::timestamp::*;

#[test]
fn converts_to_and_from_milliseconds() {
    let timestamp = TimestampMillis::new(1_700_000_000_123);

    assert_eq!(timestamp.as_millis(), 1_700_000_000_123);
    assert_eq!(u64::from(timestamp), 1_700_000_000_123);
    assert_eq!(TimestampMillis::from(1_700_000_000_123), timestamp);
    assert_eq!(
        TimestampMillis::try_from(1_700_000_000_123_i64).unwrap(),
        timestamp
    );
    assert_eq!(i64::try_from(timestamp).unwrap(), 1_700_000_000_123);
}

#[test]
fn rejects_negative_timestamps() {
    assert!(TimestampMillis::try_from(-1_i64).is_err());
}

#[test]
fn serde_round_trips_as_a_number() {
    let timestamp = TimestampMillis::new(1_700_000_000_123);
    let json = serde_json::to_string(&timestamp).unwrap();

    assert_eq!(json, "1700000000123");
    assert_eq!(
        serde_json::from_str::<TimestampMillis>(&json).unwrap(),
        timestamp
    );
}

#[test]
fn displays_and_parses_milliseconds() {
    let timestamp = TimestampMillis::new(1_700_000_000_123);

    assert_eq!(timestamp.to_string(), "1700000000123");
    assert_eq!(
        timestamp.to_string().parse::<TimestampMillis>().unwrap(),
        timestamp
    );
}

#[test]
fn supports_timestamp_arithmetic() {
    let lower = TimestampMillis::new(1_000);
    let upper = TimestampMillis::new(2_000);

    assert_eq!(upper.checked_sub(lower), Some(TimestampMillis::new(1_000)));
    assert_eq!(
        lower.checked_add(TimestampMillis::new(500)),
        Some(TimestampMillis::new(1_500))
    );
    assert_eq!(lower.checked_sub(upper), None);
}

#[test]
fn checked_arithmetic_reports_overflow_and_underflow() {
    let zero = TimestampMillis::new(0);
    let one = TimestampMillis::new(1);
    let max = TimestampMillis::new(u64::MAX);

    assert_eq!(one.checked_sub(one), Some(zero));
    assert_eq!(zero.checked_sub(one), None);
    assert_eq!(max.checked_add(one), None);
}

#[test]
fn saturating_arithmetic_clamps_to_bounds() {
    let zero = TimestampMillis::new(0);
    let one = TimestampMillis::new(1);
    let max = TimestampMillis::new(u64::MAX);

    assert_eq!(zero.saturating_sub(one), zero);
    assert_eq!(max.saturating_add(one), max);
    assert_eq!(zero.saturating_add(one), one);
    assert_eq!(max.saturating_sub(one), TimestampMillis::new(u64::MAX - 1));
}
