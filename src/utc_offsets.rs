use chrono::TimeZone;
use chrono_tz::OffsetComponents;
use chrono_tz::Tz;
use polars::prelude::*;
use polars_arrow::temporal_conversions::{
    timestamp_ms_to_datetime, timestamp_ns_to_datetime, timestamp_us_to_datetime,
};

pub(crate) fn impl_base_utc_offset(
    ca: &DatetimeChunked,
    time_unit: &TimeUnit,
    time_zone: &Tz,
) -> DurationChunked {
    let timestamp_to_datetime = match time_unit {
        TimeUnit::Nanoseconds => timestamp_ns_to_datetime,
        TimeUnit::Microseconds => timestamp_us_to_datetime,
        TimeUnit::Milliseconds => timestamp_ms_to_datetime,
    };
    ca.0.apply_values(|t| {
        let ndt = timestamp_to_datetime(t);
        let dt = time_zone.from_utc_datetime(&ndt);
        dt.offset().base_utc_offset().num_milliseconds()
    })
    .into_duration(TimeUnit::Milliseconds)
}

pub(crate) fn impl_dst_offset(
    ca: &DatetimeChunked,
    time_unit: &TimeUnit,
    time_zone: &Tz,
) -> DurationChunked {
    let timestamp_to_datetime = match time_unit {
        TimeUnit::Nanoseconds => timestamp_ns_to_datetime,
        TimeUnit::Microseconds => timestamp_us_to_datetime,
        TimeUnit::Milliseconds => timestamp_ms_to_datetime,
    };
    ca.0.apply_values(|t| {
        let ndt = timestamp_to_datetime(t);
        let dt = time_zone.from_utc_datetime(&ndt);
        dt.offset().dst_offset().num_milliseconds()
    })
    .into_duration(TimeUnit::Milliseconds)
}
