use chrono::{TimeZone, Utc};
use std::collections::BTreeMap;
use stock_trek::markets::{
    AlignedWindow, MarketAlignedWindow, MarketCandle, MarketOhlcv, RollingWindow, TimestampMillis,
};

fn millis_at(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> TimestampMillis {
    TimestampMillis::try_from(
        Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
            .timestamp_millis(),
    )
    .unwrap()
}

fn candle() -> MarketCandle {
    candle_at(0, 1.0)
}

fn ohlcv_at(open: f64) -> MarketOhlcv {
    MarketOhlcv::new(
        open,
        open + 1.0,
        open - 0.5,
        open + 0.5,
        10.0,
        15.0,
        open + 0.4,
    )
    .unwrap()
}

fn candle_at(start_time_millis_inc: u64, open: f64) -> MarketCandle {
    MarketCandle::new(
        RollingWindow::Seconds1,
        TimestampMillis::new(start_time_millis_inc),
        true,
        ohlcv_at(open),
        3,
    )
    .unwrap()
}

#[test]
fn new_rejects_duplicate_windows() {
    let market = MarketAlignedWindow::new([
        (AlignedWindow::Minutes1, vec![candle_at(2, 2.0)]),
        (AlignedWindow::Minutes1, vec![candle_at(1, 1.0)]),
    ]);

    assert!(market.is_err());
}

#[test]
fn new_rejects_duplicate_candle_intervals() {
    let market = MarketAlignedWindow::new([(
        AlignedWindow::Minutes1,
        vec![candle_at(2, 2.0), candle_at(2, 3.0)],
    )]);

    assert!(market.is_err());
}

#[test]
fn new_sorts_candles_chronologically() {
    let market = MarketAlignedWindow::new(BTreeMap::from([(
        AlignedWindow::Minutes1,
        vec![candle_at(2, 2.0), candle_at(1, 1.0)],
    )]))
    .unwrap();

    assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0, 2.0]));
}

#[test]
fn serde_json_round_trip() {
    let market =
        MarketAlignedWindow::new(BTreeMap::from([(AlignedWindow::Minutes1, vec![candle()])]))
            .unwrap();
    let json = serde_json::to_string(&market).unwrap();
    let deserialized: MarketAlignedWindow = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.opens(AlignedWindow::Minutes1), Some(vec![1.0]));
}

#[test]
fn deserialization_sorts_candles_chronologically() {
    let json = serde_json::json!({
        "candles": {
            "1m": [
                candle_at(2, 2.0),
                candle_at(1, 1.0),
            ]
        }
    });
    let deserialized: MarketAlignedWindow = serde_json::from_value(json).unwrap();

    assert_eq!(
        deserialized.opens(AlignedWindow::Minutes1),
        Some(vec![1.0, 2.0])
    );
}

#[test]
fn deserialization_rejects_duplicate_candle_intervals() {
    let json = serde_json::json!({
        "candles": {
            "1m": [
                candle_at(2, 2.0),
                candle_at(2, 3.0),
            ]
        }
    });

    assert!(serde_json::from_value::<MarketAlignedWindow>(json).is_err());
}

#[test]
fn years1_creates_candles_for_calendar_years() {
    let market = MarketAlignedWindow::years1(2024, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Years1).unwrap();

    assert_eq!(candles.len(), 1);
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2025, 1, 1, 0, 0, 0)
    );
    assert!(candles[0].is_candle_closed());
    assert_eq!(candles[0].trade_count(), 3);
}

#[test]
fn years2_rejects_years_that_do_not_start_a_window() {
    assert!(MarketAlignedWindow::years2(2025, [(ohlcv_at(1.0), 3)]).is_err());

    let market = MarketAlignedWindow::years2(2024, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Years2).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2026, 1, 1, 0, 0, 0)
    );
}

#[test]
fn months1_creates_candles_for_calendar_months() {
    let market = MarketAlignedWindow::months1(2024, 2, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Months1).unwrap();

    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 3, 1, 0, 0, 0)
    );
}

#[test]
fn month_constructors_reject_months_that_do_not_start_a_window() {
    assert!(MarketAlignedWindow::months2(2024, 2, [(ohlcv_at(1.0), 3)]).is_err());
    assert!(MarketAlignedWindow::months6(2024, 2, [(ohlcv_at(1.0), 3)]).is_err());

    let market = MarketAlignedWindow::months2(2024, 3, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Months2).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 3, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 5, 1, 0, 0, 0)
    );

    let market = MarketAlignedWindow::months6(2024, 7, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Months6).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 7, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2025, 1, 1, 0, 0, 0)
    );
}

#[test]
fn weeks1_creates_candles_for_calendar_weeks() {
    let market =
        MarketAlignedWindow::weeks1(2024, 2, 12, [(ohlcv_at(1.0), 3), (ohlcv_at(2.0), 4)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Weeks1).unwrap();

    assert_eq!(candles.len(), 2);
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 12, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 19, 0, 0, 0)
    );
    assert_eq!(
        candles[1].start_time_millis_inc(),
        millis_at(2024, 2, 19, 0, 0, 0)
    );
    assert_eq!(
        candles[1].end_time_millis_exc(),
        millis_at(2024, 2, 26, 0, 0, 0)
    );
}

#[test]
fn week_constructors_reject_dates_that_do_not_start_a_window() {
    assert!(MarketAlignedWindow::weeks1(2024, 2, 15, [(ohlcv_at(1.0), 3)]).is_err());
    assert!(MarketAlignedWindow::weeks2(2024, 2, 19, [(ohlcv_at(1.0), 3)]).is_err());
    assert!(MarketAlignedWindow::weeks4(2024, 2, 19, [(ohlcv_at(1.0), 3)]).is_err());

    let market = MarketAlignedWindow::weeks2(2024, 2, 12, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Weeks2).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 12, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 26, 0, 0, 0)
    );

    let market = MarketAlignedWindow::weeks4(2024, 2, 12, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Weeks4).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 12, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 3, 11, 0, 0, 0)
    );
}

#[test]
fn days1_creates_candles_for_calendar_days_and_rejects_invalid_dates() {
    assert!(MarketAlignedWindow::days1(2024, 2, 30, [(ohlcv_at(1.0), 3)]).is_err());

    let market = MarketAlignedWindow::days1(2024, 2, 15, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Days1).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 15, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 16, 0, 0, 0)
    );
}

#[test]
fn hours12_rejects_hours_that_do_not_start_a_window() {
    assert!(MarketAlignedWindow::hours12(2024, 2, 15, 9, [(ohlcv_at(1.0), 3)]).is_err());

    let market = MarketAlignedWindow::hours12(2024, 2, 15, 12, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Hours12).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 15, 12, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 16, 0, 0, 0)
    );
}

#[test]
fn minutes5_rejects_minutes_that_do_not_start_a_window() {
    assert!(MarketAlignedWindow::minutes5(2024, 2, 15, 12, 7, [(ohlcv_at(1.0), 3)]).is_err());

    let market = MarketAlignedWindow::minutes5(2024, 2, 15, 12, 5, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Minutes5).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 15, 12, 5, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 15, 12, 10, 0)
    );
}

#[test]
fn seconds30_rejects_seconds_that_do_not_start_a_window() {
    assert!(MarketAlignedWindow::seconds30(2024, 2, 15, 12, 30, 45, [(ohlcv_at(1.0), 3)]).is_err());

    let market =
        MarketAlignedWindow::seconds30(2024, 2, 15, 12, 30, 30, [(ohlcv_at(1.0), 3)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Seconds30).unwrap();
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 2, 15, 12, 30, 30)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 15, 12, 31, 0)
    );
}

#[test]
fn aligned_constructors_add_one_consecutive_window_per_entry() {
    let market =
        MarketAlignedWindow::years1(2024, [(ohlcv_at(1.0), 3), (ohlcv_at(2.0), 4)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Years1).unwrap();

    assert_eq!(candles.len(), 2);
    assert_eq!(
        candles[0].start_time_millis_inc(),
        millis_at(2024, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2025, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[1].start_time_millis_inc(),
        millis_at(2025, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[1].end_time_millis_exc(),
        millis_at(2026, 1, 1, 0, 0, 0)
    );
    assert_eq!(market.opens(AlignedWindow::Years1), Some(vec![1.0, 2.0]));
}

#[test]
fn calendar_window_constructors_follow_calendar_lengths() {
    let market =
        MarketAlignedWindow::months1(2024, 1, [(ohlcv_at(1.0), 3), (ohlcv_at(2.0), 4)]).unwrap();
    let candles = market.candles_for(AlignedWindow::Months1).unwrap();

    assert_eq!(
        candles[0].end_time_millis_exc(),
        millis_at(2024, 2, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[1].start_time_millis_inc(),
        millis_at(2024, 2, 1, 0, 0, 0)
    );
    assert_eq!(
        candles[1].end_time_millis_exc(),
        millis_at(2024, 3, 1, 0, 0, 0)
    );
}

#[test]
fn every_aligned_constructor_covers_exactly_its_window() {
    let ohlcv = ohlcv_at(1.0);
    let cases = vec![
        (
            MarketAlignedWindow::seconds1(2024, 2, 15, 12, 30, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Seconds1,
        ),
        (
            MarketAlignedWindow::seconds5(2024, 2, 15, 12, 30, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Seconds5,
        ),
        (
            MarketAlignedWindow::seconds10(2024, 2, 15, 12, 30, 50, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Seconds10,
        ),
        (
            MarketAlignedWindow::seconds15(2024, 2, 15, 12, 30, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Seconds15,
        ),
        (
            MarketAlignedWindow::seconds30(2024, 2, 15, 12, 30, 30, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Seconds30,
        ),
        (
            MarketAlignedWindow::minutes1(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes1,
        ),
        (
            MarketAlignedWindow::minutes2(2024, 2, 15, 12, 44, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes2,
        ),
        (
            MarketAlignedWindow::minutes3(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes3,
        ),
        (
            MarketAlignedWindow::minutes5(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes5,
        ),
        (
            MarketAlignedWindow::minutes10(2024, 2, 15, 12, 50, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes10,
        ),
        (
            MarketAlignedWindow::minutes15(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes15,
        ),
        (
            MarketAlignedWindow::minutes30(2024, 2, 15, 12, 30, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Minutes30,
        ),
        (
            MarketAlignedWindow::hours1(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours1,
        ),
        (
            MarketAlignedWindow::hours2(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours2,
        ),
        (
            MarketAlignedWindow::hours3(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours3,
        ),
        (
            MarketAlignedWindow::hours4(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours4,
        ),
        (
            MarketAlignedWindow::hours6(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours6,
        ),
        (
            MarketAlignedWindow::hours8(2024, 2, 15, 16, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours8,
        ),
        (
            MarketAlignedWindow::hours12(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Hours12,
        ),
        (
            MarketAlignedWindow::days1(2024, 2, 15, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Days1,
        ),
        (
            MarketAlignedWindow::weeks1(2024, 2, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Weeks1,
        ),
        (
            MarketAlignedWindow::weeks2(2024, 2, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Weeks2,
        ),
        (
            MarketAlignedWindow::weeks4(2024, 2, 12, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Weeks4,
        ),
        (
            MarketAlignedWindow::months1(2024, 2, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Months1,
        ),
        (
            MarketAlignedWindow::months2(2024, 3, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Months2,
        ),
        (
            MarketAlignedWindow::months3(2024, 4, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Months3,
        ),
        (
            MarketAlignedWindow::months4(2024, 5, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Months4,
        ),
        (
            MarketAlignedWindow::months6(2024, 7, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Months6,
        ),
        (
            MarketAlignedWindow::years1(2024, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Years1,
        ),
        (
            MarketAlignedWindow::years2(2024, [(ohlcv.clone(), 3)]).unwrap(),
            AlignedWindow::Years2,
        ),
    ];

    for (market, window) in cases {
        let candles = market.candles_for(window).unwrap();
        assert_eq!(candles.len(), 1, "unexpected candles for {window}");
        if let Some(exact_millis) = window.millis_exact() {
            assert_eq!(
                candles[0].duration_millis(),
                exact_millis,
                "unexpected duration for {window}"
            );
        }
    }
}
