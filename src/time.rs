//! Perf UI Entries for displaying the current time.

use bevy::prelude::*;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::utils::Duration;

use crate::prelude::*;
use crate::utils::*;

/// Perf UI Entry to display the time the Bevy app has been running.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryRunningTime {
    /// If set, count time relative to this.
    /// If unset, count time since app startup.
    /// (represented as a duration since startup, as per Bevy's `Time::elapsed()`)
    ///
    /// Default: `None`
    pub start: Option<Duration>,
    /// If true, format time as HH:MM:SS (with optional fractional part as per `precision`).
    /// If false, format time as seconds.
    ///
    /// Default: `false`
    pub format_hms: bool,
    /// Display the unit ("s") alongside the number.
    ///
    /// Only used if `format_hms = false`.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Only used if `format_hms = false`.
    ///
    /// Default: `5`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `3`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryRunningTime {
    fn default() -> Self {
        PerfUiEntryRunningTime {
            start: None,
            format_hms: false,
            display_units: true,
            digits: 5,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display the wall clock / current time of day (system time).
///
/// This time is in UTC, unless you enable the optional `chrono` dependency on
/// this crate. If `chrono` is enabled, it will be in local time.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryClock {
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryClock {
    fn default() -> Self {
        PerfUiEntryClock {
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryRunningTime {
    type Value = Duration;
    type SystemParam = SRes<Time>;

    fn label(&self) -> &str {
        "Running Time"
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value<'w>(
        &mut self,
        time: &mut <Self::SystemParam as SystemParam>::Item<'w, '_>,
    ) -> Option<Self::Value> {
        let elapsed = time.elapsed();
        if let Some(start) = self.start {
            Some(elapsed - start)
        } else {
            Some(elapsed)
        }
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        if self.format_hms {
            format_pretty_time(self.precision, *value)
        } else {
            let mut s = format_pretty_float(self.digits, self.precision, value.as_secs_f64());
            if self.display_units {
                s.push_str(" s");
            }
            s
        }
    }
}

impl PerfUiEntry for PerfUiEntryClock {
    // (h, m, s, nanos)
    type Value = (u32, u32, u32, u32);
    type SystemParam = ();

    fn label(&self) -> &str {
        if cfg!(feature = "chrono") {
            "Clock"
        } else {
            "Clock (UTC)"
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value<'w>(
        &mut self,
        _: &mut <Self::SystemParam as SystemParam>::Item<'w, '_>,
    ) -> Option<Self::Value> {
        #[cfg(feature = "chrono")]
        {
            use chrono::Timelike;
            let now = chrono::Local::now();
            let h = now.hour();
            let m = now.minute();
            let s = now.second();
            let nanos = now.timestamp_subsec_nanos();
            Some((h as u32, m as u32, s as u32, nanos))
        }
        #[cfg(not(feature = "chrono"))]
        {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?;
            let secs = now.as_secs();
            let h = (secs / 3600) % 24;
            let m = (secs / 60) % 60;
            let s = secs % 60;
            let nanos = now.subsec_nanos();
            Some((h as u32, m as u32, s as u32, nanos))
        }
    }
    fn format_value(
        &self,
        &(h, m, s, nanos): &Self::Value,
    ) -> String {
        format_pretty_time_hms(self.precision, h, m, s, nanos)
    }
}
