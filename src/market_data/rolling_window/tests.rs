use super::*;
use crate::market_data::window_test_support::window_tests;

window_tests!(
    RollingWindow,
    RollingWindow::Seconds1 => "1s",
    RollingWindow::Hours24 => "24h",
    RollingWindow::Days7 => "7d",
);
