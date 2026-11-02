use super::*;
use crate::market_data::window_test_support::{CheckedWindow, window_tests};

window_tests!(
    RollingWindow,
    RollingWindow::Seconds1 => "1s",
    RollingWindow::Days1 => "1d",
    RollingWindow::Weeks1 => "1w",
);
