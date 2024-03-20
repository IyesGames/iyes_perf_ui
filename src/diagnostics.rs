//! Perf UI Entries based on Bevy Diagnostics

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::utils::FloatOrd;

use crate::prelude::*;
use crate::utils::*;

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFPS {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// If FPS is less than this value, text will be colored RED.
    ///
    /// Default: `30.0`
    pub threshold_bad: f32,
    /// If FPS is at this value, text will be colored YELLOW.
    ///
    /// Between bad and normal, will gradually transition from red to yellow.
    ///
    /// Between normal and good, will gradually transition from yellow to green.
    ///
    /// Default: `60.0`
    pub threshold_normal: f32,
    /// If FPS is greater than this value, text will be colored GREEN.
    ///
    /// Default: `120.0`
    pub threshold_good: f32,
    /// If FPS is below this value, use highlight font.
    ///
    /// Default: `20.0`
    pub threshold_highlight: f32,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `4`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFPS {
    fn default() -> Self {
        PerfUiEntryFPS {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            threshold_good: 120.0,
            threshold_normal: 60.0,
            threshold_bad: 30.0,
            threshold_highlight: 20.0,
            smoothed: true,
            digits: 4,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
///
/// Displays the worst (lowest) value in recent history.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFPSWorst {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// If FPS is less than this value, text will be colored RED.
    ///
    /// Default: `30.0`
    pub threshold_bad: f32,
    /// If FPS is at this value, text will be colored YELLOW.
    ///
    /// Between bad and normal, will gradually transition from red to yellow.
    ///
    /// Between normal and good, will gradually transition from yellow to green.
    ///
    /// Default: `60.0`
    pub threshold_normal: f32,
    /// If FPS is greater than this value, text will be colored GREEN.
    ///
    /// Default: `120.0`
    pub threshold_good: f32,
    /// If FPS is below this value, use highlight font.
    ///
    /// Default: `20.0`
    pub threshold_highlight: f32,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `4`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFPSWorst {
    fn default() -> Self {
        PerfUiEntryFPSWorst {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            threshold_good: 120.0,
            threshold_normal: 60.0,
            threshold_bad: 30.0,
            threshold_highlight: 20.0,
            digits: 4,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame time measurement diagnostic.
///
/// Displays the frame time in *milliseconds*.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFrameTime {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// Display the unit ("ms") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// If greater than this value, text will be colored RED.
    ///
    /// Default: frame time of 30 FPS
    pub threshold_bad: f32,
    /// If at this value, text will be colored YELLOW.
    ///
    /// Between bad and normal, will gradually transition from red to yellow.
    ///
    /// Between normal and good, will gradually transition from yellow to green.
    ///
    /// Default: frame time of 60 FPS
    pub threshold_normal: f32,
    /// If less than this value, text will be colored GREEN.
    ///
    /// Default: frame time of 120 FPS
    pub threshold_good: f32,
    /// If above this value, use highlight font.
    ///
    /// Default: frame time of 20 FPS
    pub threshold_highlight: f32,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
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

impl Default for PerfUiEntryFrameTime {
    fn default() -> Self {
        PerfUiEntryFrameTime {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            display_units: true,
            threshold_good: 1000.0 / 120.0,
            threshold_normal: 1000.0 / 60.0,
            threshold_bad: 1000.0 / 30.0,
            threshold_highlight: 1000.0 / 20.0,
            smoothed: true,
            digits: 2,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame time measurement diagnostic.
///
/// Displays the worst (highest) value in recent history.
///
/// Displays the frame time in *milliseconds*.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFrameTimeWorst {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// Display the unit ("ms") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// If greater than this value, text will be colored RED.
    ///
    /// Default: frame time of 30 FPS
    pub threshold_bad: f32,
    /// If at this value, text will be colored YELLOW.
    ///
    /// Between bad and normal, will gradually transition from red to yellow.
    ///
    /// Between normal and good, will gradually transition from yellow to green.
    ///
    /// Default: frame time of 60 FPS
    pub threshold_normal: f32,
    /// If less than this value, text will be colored GREEN.
    ///
    /// Default: frame time of 120 FPS
    pub threshold_good: f32,
    /// If above this value, use highlight font.
    ///
    /// Default: frame time of 20 FPS
    pub threshold_highlight: f32,
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

impl Default for PerfUiEntryFrameTimeWorst {
    fn default() -> Self {
        PerfUiEntryFrameTimeWorst {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            display_units: true,
            threshold_good: 1000.0 / 120.0,
            threshold_normal: 1000.0 / 60.0,
            threshold_bad: 1000.0 / 30.0,
            threshold_highlight: 1000.0 / 20.0,
            digits: 2,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame counter.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFrameCount {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Number of digits to display.
    ///
    /// Default: `6`
    pub digits: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFrameCount {
    fn default() -> Self {
        PerfUiEntryFrameCount {
            label: String::new(),
            digits: 6,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in ECS entity counter.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryEntityCount {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// If greater than this value, text will be colored RED.
    ///
    /// Default: `10000`
    pub threshold_high: u32,
    /// If at this value, text will be colored YELLOW.
    ///
    /// Between low and normal, will gradually transition from green to yellow.
    ///
    /// Between normal and high, will gradually transition from yellow to red.
    ///
    /// Default: `1000`
    pub threshold_normal: u32,
    /// If less than this value, text will be colored GREEN.
    ///
    /// Default: `100`
    pub threshold_low: u32,
    /// If above this value, use highlight font.
    ///
    /// Default: `20000`
    pub threshold_highlight: u32,
    /// Number of digits to display.
    ///
    /// Default: `6`
    pub digits: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryEntityCount {
    fn default() -> Self {
        PerfUiEntryEntityCount {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            threshold_high: 10000,
            threshold_normal: 1000,
            threshold_low: 100,
            threshold_highlight: 20000,
            digits: 6,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in CPU Usage measurement diagnostic.
///
/// Displays the usage as a percentage.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryCpuUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// If greater than this value, text will be colored RED.
    ///
    /// Default: 75.0 %
    pub threshold_high: f32,
    /// If at this value, text will be colored YELLOW.
    ///
    /// Between low and normal, will gradually transition from green to yellow.
    ///
    /// Between normal and high, will gradually transition from yellow to red.
    ///
    /// Default: 50.0 %
    pub threshold_normal: f32,
    /// If less than this value, text will be colored GREEN.
    ///
    /// Default: 25.0 %
    pub threshold_low: f32,
    /// If above this value, use highlight font.
    ///
    /// Default: 90.0 %
    pub threshold_highlight: f32,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryCpuUsage {
    fn default() -> Self {
        PerfUiEntryCpuUsage {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            threshold_high: 75.0,
            threshold_normal: 50.0,
            threshold_low: 25.0,
            threshold_highlight: 90.0,
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in Memory (RAM) Usage measurement diagnostic.
///
/// Displays the usage as a percentage.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryMemUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value
    ///
    /// Default: `true`
    pub enable_color: bool,
    /// Enable highlighting based on value
    ///
    /// Default: `true`
    pub enable_highlight: bool,
    /// If greater than this value, text will be colored RED.
    ///
    /// Default: 75.0 %
    pub threshold_high: f32,
    /// If at this value, text will be colored YELLOW.
    ///
    /// Between low and normal, will gradually transition from green to yellow.
    ///
    /// Between normal and high, will gradually transition from yellow to red.
    ///
    /// Default: 50.0 %
    pub threshold_normal: f32,
    /// If less than this value, text will be colored GREEN.
    ///
    /// Default: 25.0 %
    pub threshold_low: f32,
    /// If above this value, use highlight font.
    ///
    /// Default: 90.0 %
    pub threshold_highlight: f32,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryMemUsage {
    fn default() -> Self {
        PerfUiEntryMemUsage {
            label: String::new(),
            enable_color: true,
            enable_highlight: true,
            threshold_high: 75.0,
            threshold_normal: 50.0,
            threshold_low: 25.0,
            threshold_highlight: 90.0,
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryFPS {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?.smoothed()?
        } else {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_down(self.threshold_bad, self.threshold_normal, self.threshold_good, *value))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value as f32 <= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryFPSWorst {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS (min)"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?
            .values()
            .filter_map(|f| if !f.is_nan() {
                Some(FloatOrd(*f as f32))
            } else {
                None
            })
            .min()?.0
        )
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value as f64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_down(self.threshold_bad, self.threshold_normal, self.threshold_good, *value as f64))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value <= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryFrameTime {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Frame Time"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)?.smoothed()?
        } else {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" ms");
        }
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_up(self.threshold_good, self.threshold_normal, self.threshold_bad, *value))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value as f32 >= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryFrameTimeWorst {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Frame Time (max)"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)?
            .values()
            .filter_map(|f| if !f.is_nan() {
                Some(FloatOrd(*f as f32))
            } else {
                None
            })
            .max()?.0
        )
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(self.digits, self.precision, *value as f64);
        if self.display_units {
            s.push_str(" ms");
        }
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_up(self.threshold_good, self.threshold_normal, self.threshold_bad, *value as f64))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value >= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryFrameCount {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = u32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Frame Count"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)?.value()? as u32)
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_int(self.digits, *value as i64)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryEntityCount {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = u32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Entity Count"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)?.value()? as u32)
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_int(self.digits, *value as i64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_up(
            self.threshold_low as f32,
            self.threshold_normal as f32,
            self.threshold_high as f32,
            *value as f64,
        ))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value >= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryCpuUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Total CPU Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_up(self.threshold_low, self.threshold_normal, self.threshold_high, *value))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value as f32 >= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryMemUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Total RAM Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &mut self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if !self.enable_color {
            return None;
        }
        Some(ryg_gradient_up(self.threshold_low, self.threshold_normal, self.threshold_high, *value))
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.enable_highlight && *value as f32 >= self.threshold_highlight
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}
