use chrono::{DateTime, Duration, Months, Utc};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, FromRepr, Serialize, Deserialize)]
#[non_exhaustive]
#[repr(u8)]
pub enum AlignedWindow {
    Seconds5,
    Seconds10,
    Seconds15,
    Seconds30,
    Seconds60,
    Minutes1,
    Minutes2,
    Minutes3,
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes30,
    Minutes60,
    Hours1,
    Hours2,
    Hours3,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Hours24,
    Days1,
    Days7,
    Weeks1,
    Weeks2,
    Weeks4,
    Months1,
    Months2,
    Months3,
    Months4,
    Months6,
    Months12,
    Years1,
    Years2,
}

impl AlignedWindow {
    pub fn checked_add(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            AlignedWindow::Seconds5 => Some(date_time + Duration::seconds(5)),
            AlignedWindow::Seconds10 => Some(date_time + Duration::seconds(10)),
            AlignedWindow::Seconds15 => Some(date_time + Duration::seconds(15)),
            AlignedWindow::Seconds30 => Some(date_time + Duration::seconds(30)),
            AlignedWindow::Seconds60 => Some(date_time + Duration::seconds(60)),
            AlignedWindow::Minutes1 => Some(date_time + Duration::minutes(1)),
            AlignedWindow::Minutes2 => Some(date_time + Duration::minutes(2)),
            AlignedWindow::Minutes3 => Some(date_time + Duration::minutes(3)),
            AlignedWindow::Minutes5 => Some(date_time + Duration::minutes(5)),
            AlignedWindow::Minutes10 => Some(date_time + Duration::minutes(10)),
            AlignedWindow::Minutes15 => Some(date_time + Duration::minutes(15)),
            AlignedWindow::Minutes30 => Some(date_time + Duration::minutes(30)),
            AlignedWindow::Minutes60 => Some(date_time + Duration::minutes(60)),
            AlignedWindow::Hours1 => Some(date_time + Duration::hours(5)),
            AlignedWindow::Hours2 => Some(date_time + Duration::hours(2)),
            AlignedWindow::Hours3 => Some(date_time + Duration::hours(3)),
            AlignedWindow::Hours4 => Some(date_time + Duration::hours(4)),
            AlignedWindow::Hours6 => Some(date_time + Duration::hours(6)),
            AlignedWindow::Hours8 => Some(date_time + Duration::hours(8)),
            AlignedWindow::Hours12 => Some(date_time + Duration::hours(12)),
            AlignedWindow::Hours24 => Some(date_time + Duration::hours(24)),
            AlignedWindow::Days1 => Some(date_time + Duration::days(1)),
            AlignedWindow::Days7 => Some(date_time + Duration::days(7)),
            AlignedWindow::Weeks1 => Some(date_time + Duration::weeks(1)),
            AlignedWindow::Weeks2 => Some(date_time + Duration::weeks(2)),
            AlignedWindow::Weeks4 => Some(date_time + Duration::weeks(4)),
            AlignedWindow::Months1 => date_time.checked_add_months(Months::new(1)),
            AlignedWindow::Months2 => date_time.checked_add_months(Months::new(2)),
            AlignedWindow::Months3 => date_time.checked_add_months(Months::new(3)),
            AlignedWindow::Months4 => date_time.checked_add_months(Months::new(4)),
            AlignedWindow::Months6 => date_time.checked_add_months(Months::new(6)),
            AlignedWindow::Months12 => date_time.checked_add_months(Months::new(12)),
            AlignedWindow::Years1 => date_time.checked_add_months(Months::new(12)),
            AlignedWindow::Years2 => date_time.checked_add_months(Months::new(24)),
        }
    }
}
