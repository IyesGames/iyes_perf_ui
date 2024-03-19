//! Helper functions
//!
//! Mostly stuff for implementing new `PerfUiEntry` types and
//! formatting of values.

use std::sync::atomic::{AtomicI32, Ordering};

use bevy::prelude::*;
use bevy::utils::Duration;

static NEXT_SORT_KEY: AtomicI32 = AtomicI32::new(1);

/// Generate a new incrementally-increasing sort key.
///
/// Useful for `Default` impls of `PerfUiEntry` types, so
/// that whenever a user constructs a new entry, it always
/// appears after any previously-constructed entries.
pub fn next_sort_key() -> i32 {
    NEXT_SORT_KEY.fetch_add(1, Ordering::SeqCst)
}

/// Generate a Red->Yellow->Green gradient from Low->Mid->High
pub fn ryg_gradient_down(low: f32, mid: f32, high: f32, value: f64) -> Color {
    let value = value as f32;
    if value >= high {
        Color::GREEN
    } else if value >= mid {
        let n = value - mid;
        let d = high - mid;
        Color::rgb(
            1.0 - n / d,
            1.0,
            0.0,
        )
    } else if value >= low {
        let n = value - low;
        let d = mid - low;
        Color::rgb(
            1.0,
            n / d,
            0.0,
        )
    } else {
        Color::RED
    }
}

/// Generate a Green->Yellow->Red gradient from Low->Mid->High
pub fn ryg_gradient_up(low: f32, mid: f32, high: f32, value: f64) -> Color {
    let value = value as f32;
    if value >= high {
        Color::RED
    } else if value >= mid {
        let n = value - mid;
        let d = high - mid;
        Color::rgb(
            1.0,
            1.0 - n / d,
            0.0,
        )
    } else if value >= low {
        let n = value - low;
        let d = mid - low;
        Color::rgb(
            n / d,
            1.0,
            0.0,
        )
    } else {
        Color::GREEN
    }
}

/// Format a float in a pretty way.
///
/// - Right aligned
/// - Padded with spaces to accomodate total width (digits + precision + decimal point (if any))
/// - Clamped to all 9s if above digits count (example: `99.999` if digits = 2 and precision = 3)
pub fn format_pretty_float(digits: u8, precision: u8, mut value: f64) -> String {
    let digits = digits.max(1);
    let max = 10.0f64.powi(digits as i32);

    if value >= max {
        value = max - 10.0f64.powi(-(precision as i32));
    }

    let width = if precision > 0 {
        digits + precision + 1
    } else {
        digits
    };

    format!(
        "{number:>width$.prec$}",
        number = value,
        width = width as usize,
        prec = precision as usize,
    )
}

/// Format an integer in a pretty way.
///
/// - Right aligned
/// - Padded with spaces to accomodate total width (digits)
/// - Clamped to all 9s if above digits count (example: `99` if digits = 2)
pub fn format_pretty_int(digits: u8, mut value: i64) -> String {
    let width = if value < 0 {
        let digits = digits.max(2) - 1;
        let max = 10i64.pow(digits as u32);
        if -value >= max {
            value = -(max - 1);
        }
        digits + 1
    } else {
        let digits = digits.max(1);
        let max = 10i64.pow(digits as u32);
        if value >= max {
            value = max - 1;
        }
        digits
    };

    format!(
        "{number:>width$}",
        number = value,
        width = width as usize,
    )
}

/// Format time in a pretty way.
///
/// - Right aligned
/// - `HH:MM:SS.mmm` (hours, minutes, seconds, milliseconds)
/// - Milliseconds optional (precision = 0 to disable)
/// - Automatically omits hours and minutes if they would be zero
/// - Padded with spaces to accomodate maximum width
/// - Clamped to all 9s if above max (example: `99:59:59.999`)
pub fn format_pretty_time(precision: u8, value: Duration) -> String {
    let max = 99 * 3600 + 59 * 60 + 59;
    let secs = value.as_secs();
    if secs > max {
        if precision > 0 {
            return format!("99:59:59.{dummy:9<prec$}", dummy = "", prec = precision as usize);
        } else {
            return "99:59:59".into()
        }
    }
    let secs = secs as u32;
    format_pretty_time_hms(precision, secs / 3600, secs / 60, secs, value.subsec_nanos())
}

pub fn format_pretty_time_hms(precision: u8, h: u32, m: u32, s: u32, nanos: u32) -> String {
    // sanitize
    let hrs = h % 100;
    let mins = m % 60;
    let secs = s % 60;
    let frac = nanos / 10u32.pow(9 - (precision as u32).min(9));
    if precision > 0 {
        if hrs > 0 {
            format!("{:2}:{:02}:{:02}.{:0w$}", hrs, mins, secs, frac, w = precision as usize)
        } else if mins > 0 {
            format!("{:5}:{:02}.{:0w$}", mins, secs, frac, w = precision as usize)
        } else {
            format!("{:8}.{:0w$}", secs, frac, w = precision as usize)
        }
    } else {
        if hrs > 0 {
            format!("{:2}:{:02}:{:02}", hrs, mins, secs)
        } else if mins > 0 {
            format!("{:5}:{:02}", mins, secs)
        } else {
            format!("{:8}", secs)
        }
    }
}
