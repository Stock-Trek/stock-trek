mod common;

use common::{CheckedWindow, window_tests};
use stock_trek::markets::RollingWindow;

window_tests!(
    RollingWindow,
    RollingWindow::Seconds1 => "1s",
    RollingWindow::Days1 => "1d",
    RollingWindow::Weeks1 => "1w",
);
