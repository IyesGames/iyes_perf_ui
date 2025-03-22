//! Helper functions
//!
//! Mostly stuff for implementing new `PerfUiEntry` types and
//! formatting of values.

use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;

use bevy::prelude::*;
use bevy::math::FloatOrd;

static NEXT_SORT_KEY: AtomicI32 = AtomicI32::new(1);

/// Generate a new incrementally-increasing sort key.
///
/// Useful for `Default` impls of `PerfUiEntry` types, so
/// that whenever a user constructs a new entry, it always
/// appears after any previously-constructed entries.
pub fn next_sort_key() -> i32 {
    NEXT_SORT_KEY.fetch_add(1, Ordering::Relaxed)
}

/// Represents a color gradient with any number of stops.
///
/// Each "stop" is a predefined color associated with a specific value.
///
/// You can then interpolate based on an arbitrary value, to get a
/// smoothly-varying color.
///
/// The interpolation is done in Bevy's OKLAB color space, so it looks
/// nicer and more perceputally-uniform.
#[derive(Debug, Default, Clone)]
pub struct ColorGradient {
    stops: Vec<(FloatOrd, Oklaba)>,
}

impl ColorGradient {
    /// Create a new empty gradient.
    ///
    /// Use the `with_stop`/`with_stops` builder methods to conveniently
    /// add your desired stops.
    ///
    /// If you don't add any stops, `get_color_for_value` will always
    /// return `None`.
    pub fn new() -> Self {
        ColorGradient {
            stops: vec![],
        }
    }

    /// Create a "gradient" with only one color.
    ///
    /// This color will be used for all values.
    pub fn single(color: Color) -> Self {
        ColorGradient {
            stops: vec![
                (FloatOrd(f32::NEG_INFINITY), color.into()),
            ],
        }
    }

    /// Preset constructor: Red-Yellow-Green between the specified low-mid-high values.
    pub fn new_preset_ryg(low: f32, mid: f32, high: f32) -> Result<Self, ()> {
        if low.is_nan() || mid.is_nan() || high.is_nan() || low > mid || mid > high {
            return Err(());
        }
        Ok(ColorGradient {
            stops: vec![
                (FloatOrd(low), Color::srgb(1.0, 0.0, 0.0).into()),
                (FloatOrd(mid), Color::srgb(1.0, 1.0, 0.0).into()),
                (FloatOrd(high), Color::srgb(0.0, 1.0, 0.0).into()),
            ],
        })
    }

    /// Preset constructor: Green-Yellow-Red between the specified low-mid-high values.
    pub fn new_preset_gyr(low: f32, mid: f32, high: f32) -> Result<Self, ()> {
        if low.is_nan() || mid.is_nan() || high.is_nan() || low > mid || mid > high {
            return Err(());
        }
        Ok(ColorGradient {
            stops: vec![
                (FloatOrd(low), Color::srgb(0.0, 1.0, 0.0).into()),
                (FloatOrd(mid), Color::srgb(1.0, 1.0, 0.0).into()),
                (FloatOrd(high), Color::srgb(1.0, 0.0, 0.0).into()),
            ],
        })
    }

    /// Add a stop to the gradient.
    ///
    /// See `with_stop` for a builder-style version of this method.
    pub fn add_stop(&mut self, value: f32, color: Color) {
        // NaN values are nonsensical, avoid getting them into our Vec
        if value.is_nan() {
            return;
        }
        let stop = (FloatOrd(value), color.into());

        // ensure our Vec is always in sorted order
        match self.stops.binary_search_by_key(&stop.0, |x| x.0) {
            Ok(i) => {
                // replace existing stop
                self.stops[i].1 = stop.1;
            }
            Err(i) => {
                // add new stop
                self.stops.insert(i, stop);
            }
        }
    }

    /// Add multiple stops to the gradient.
    ///
    /// See `with_stops` for a builder-style version of this method.
    pub fn add_stops<Iter, Item>(&mut self, stops: Iter)
    where
        Item: Into<(f32, Color)>,
        Iter: IntoIterator<Item = Item>,
    {
        for stop in stops {
            let stop = stop.into();
            self.add_stop(stop.0, stop.1);
        }
    }

    /// Add a stop to the gradient (builder-style API).
    ///
    /// See `add_stop` for a non-builder-style version of this method.
    pub fn with_stop(mut self, value: f32, color: Color) -> Self {
        self.add_stop(value, color);
        self
    }

    /// Add multiple stops to the gradient (builder-style API).
    ///
    /// See `add_stops` for a non-builder-style version of this method.
    pub fn with_stops<Iter, Item>(mut self, stops: Iter) -> Self
    where
        Item: Into<(f32, Color)>,
        Iter: IntoIterator<Item = Item>,
    {
        self.add_stops(stops);
        self
    }

    /// Perform the actual interpolation. Return a color for the provided value.
    ///
    /// If the value is above the highest stop, the highest stop's color is returned.
    ///
    /// If the value is below the lowest stop, the lowest stop's color is returned.
    ///
    /// If the value is in-between, a color interpolated between the nearest two stops'
    /// colors is returned.
    ///
    /// If the gradient is empty (no stops were added), returns `None`.
    pub fn get_color_for_value(&self, value: f32) -> Option<Color> {
        if value.is_nan() {
            return None;
        }
        let value = FloatOrd(value);

        let first_stop = self.stops.first()?;
        let last_stop = self.stops.last()?;

        if value >= last_stop.0 {
            return Some(last_stop.1.into());
        }
        if value <= first_stop.0 {
            return Some(first_stop.1.into());
        }

        match self.stops.binary_search_by_key(&value, |x| x.0) {
            Ok(i) => {
                Some(self.stops[i].1.into())
            }
            Err(i) => {
                let stop_low = self.stops[i - 1];
                let stop_high = self.stops[i];
                let lerp_value = (value.0 - stop_low.0.0) / (stop_high.0.0 - stop_low.0.0);
                Some(stop_low.1.mix(&stop_high.1, lerp_value).into())
            }
        }
    }

    /// Get the first (lowest) stop of the gradient
    pub fn min_stop(&self) -> Option<(&f32, &Oklaba)> {
        self.stops.first().map(|(f, c)| (&f.0, c))
    }

    /// Get the last (highest) stop of the gradient
    pub fn max_stop(&self) -> Option<(&f32, &Oklaba)> {
        self.stops.last().map(|(f, c)| (&f.0, c))
    }

    /// Mutate the first (lowest) stop of the gradient
    pub fn min_stop_mut(&mut self) -> Option<(&mut f32, &mut Oklaba)> {
        self.stops.first_mut().map(|(f, c)| (&mut f.0, c))
    }

    /// Mutate the last (highest) stop of the gradient
    pub fn max_stop_mut(&mut self) -> Option<(&mut f32, &mut Oklaba)> {
        self.stops.last_mut().map(|(f, c)| (&mut f.0, c))
    }

    /// Iterate over all the stops of the gradient
    pub fn iter_stops(&self) -> impl Iterator<Item = (&f32, &Oklaba)> {
        self.stops.iter().map(|(f, c)| {
            (&f.0, c)
        })
    }

    /// Iterate mutably over all the stops of the gradient
    pub fn iter_stops_mut(&mut self) -> impl Iterator<Item = (&mut f32, &mut Oklaba)> {
        self.stops.iter_mut().map(|(f, c)| {
            (&mut f.0, c)
        })
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

    format!(
        "{number:.prec$}",
        number = value,
        prec = precision as usize,
    )
}

/// Format an integer in a pretty way.
///
/// - Right aligned
/// - Padded with spaces to accomodate total width (digits)
/// - Clamped to all 9s if above digits count (example: `99` if digits = 2)
pub fn format_pretty_int(digits: u8, mut value: i64) -> String {
    if value < 0 {
        let digits = digits.max(2) - 1;
        let max = 10i64.pow(digits as u32);
        if -value >= max {
            value = -(max - 1);
        }
    } else {
        let digits = digits.max(1);
        let max = 10i64.pow(digits as u32);
        if value >= max {
            value = max - 1;
        }
    };

    format!(
        "{number}",
        number = value,
    )
}

/// Format a time duration in a pretty way.
///
/// See [`format_pretty_time_hms`].
///
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

/// Format time (provided as hours, minutes, seconds, nanoseconds) in a pretty way.
///
/// - Right aligned
/// - `HH:MM:SS.f*` (hours, minutes, seconds, fractional seconds)
/// - Fractional part optional (precision = 0 to disable)
/// - Automatically omits hours and minutes if they would be zero
/// - Padded with spaces to accomodate maximum width
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
