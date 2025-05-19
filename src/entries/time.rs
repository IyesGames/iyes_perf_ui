//! Perf UI Entries for displaying the current time.

use bevy::prelude::*;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use std::time::Duration;

use crate::prelude::*;
use crate::entry::*;
use crate::utils::*;

/// Perf UI Entry to display the time the Bevy app has been running.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryRunningTime {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
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
            label: String::new(),
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
#[require(PerfUiRoot)]
pub struct PerfUiEntryClock {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// If true, time will be displayed in UTC and not the local timezone.
    ///
    /// If the `chrono` cargo feature is disabled, time will always be displayed
    /// in UTC regardless of this setting.
    ///
    /// Default: `false`
    pub prefer_utc: bool,
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
            label: String::new(),
            prefer_utc: false,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's Fixed Time Step duration.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFixedTimeStep {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the unit ("ms" or "Hz") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Display it as a rate in Hz instead of duration in milliseconds.
    ///
    /// Default: `true`
    pub as_hz: bool,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `3`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFixedTimeStep {
    fn default() -> Self {
        PerfUiEntryFixedTimeStep {
            label: String::new(),
            display_units: true,
            as_hz: true,
            digits: 3,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's Fixed Time overstep.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFixedOverstep {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the unit ("ms" or "%") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Display it as a percentage of the timestep instead of duration in milliseconds.
    ///
    /// Default: `true`
    pub as_percent: bool,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `2`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `3`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFixedOverstep {
    fn default() -> Self {
        PerfUiEntryFixedOverstep {
            label: String::new(),
            display_units: true,
            as_percent: true,
            digits: 3,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryRunningTime {
    type Value = Duration;
    type SystemParam = SRes<Time<Real>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Running Time"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        time: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
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
        if self.label.is_empty() {
            if cfg!(feature = "chrono") && !self.prefer_utc {
                "Clock"
            } else {
                "Clock (UTC)"
            }
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        _: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        #[cfg(feature = "chrono")]
        if !self.prefer_utc {
            return get_system_clock_local();
        }

        get_system_clock_utc()
    }
    fn format_value(
        &self,
        &(h, m, s, nanos): &Self::Value,
    ) -> String {
        format_pretty_time_hms(self.precision, h, m, s, nanos)
    }
}

impl PerfUiEntry for PerfUiEntryFixedTimeStep {
    type Value = Duration;
    type SystemParam = SRes<Time<Fixed>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Fixed Time Step"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        time: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(time.timestep())
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let (unit, value) = if self.as_hz {
            (" Hz", 1_000_000_000f64 / value.as_nanos() as f64)
        } else {
            (" ms", value.as_nanos() as f64 / 1_000_000f64)
        };
        let mut s = format_pretty_float(self.digits, self.precision, value);
        if self.display_units {
            s.push_str(unit);
        }
        s
    }
}

impl PerfUiEntry for PerfUiEntryFixedOverstep {
    type Value = f64;
    type SystemParam = SRes<Time<Fixed>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Fixed Overstep"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        time: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.as_percent {
            time.overstep_fraction_f64() * 100.0
        } else {
            time.overstep().as_secs_f64() * 1000.0
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(self.digits, self.precision, *value);
        if self.as_percent {
            s.push('%');
        } else if self.display_units {
            s.push_str(" ms");
        }
        s
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFixedOverstep {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "chrono")]
fn get_system_clock_local() -> Option<(u32, u32, u32, u32)> {
    use chrono::Timelike;
    let now = chrono::Local::now();
    let h = now.hour();
    let m = now.minute();
    let s = now.second();
    let nanos = now.timestamp_subsec_nanos();
    Some((h, m, s, nanos))
}

fn get_system_clock_utc() -> Option<(u32, u32, u32, u32)> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?;
    let secs = now.as_secs();
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    let nanos = now.subsec_nanos();
    Some((h as u32, m as u32, s as u32, nanos))
}
