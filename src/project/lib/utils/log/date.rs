use chrono::{FixedOffset, TimeZone};
use git2::Time;

pub fn date(time: &Time) -> String {
    let tz = FixedOffset::east_opt(time.offset_minutes() * 60).unwrap();
    let dt = tz.timestamp_opt(time.seconds(), 0).unwrap();

    dt.format("%Y/%m/%d %H:%M:%S %z").to_string()
}