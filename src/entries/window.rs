//! Perf UI Entries for info about application windows.

use bevy::prelude::*;
use bevy::ecs::system::lifetimeless::SQuery;
use bevy::ecs::system::SystemParam;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;

use crate::prelude::*;
use crate::entry::*;
use crate::utils::*;

/// Perf UI Entry to display the window mode (windowed, fullscreen, etc).
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryWindowMode {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the value from the specified window (in a multi-window application).
    ///
    /// If `None` (the default), the primary window is selected.
    pub window: Option<Entity>,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryWindowMode {
    fn default() -> Self {
        PerfUiEntryWindowMode {
            label: String::new(),
            window: None,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display the window present mode (vsync).
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryWindowPresentMode {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the value from the specified window (in a multi-window application).
    ///
    /// If `None` (the default), the primary window is selected.
    pub window: Option<Entity>,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryWindowPresentMode {
    fn default() -> Self {
        PerfUiEntryWindowPresentMode {
            label: String::new(),
            window: None,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display the window size / resolution.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryWindowScaleFactor {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the value from the specified window (in a multi-window application).
    ///
    /// If `None` (the default), the primary window is selected.
    pub window: Option<Entity>,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `2`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryWindowScaleFactor {
    fn default() -> Self {
        PerfUiEntryWindowScaleFactor {
            label: String::new(),
            window: None,
            digits: 2,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display the window size / resolution.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryWindowResolution {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Separate the X and Y values by this string.
    ///
    /// Default: `"x"`.
    pub separator: &'static str,
    /// Display the unit ("px") alongside the numbers.
    ///
    /// Default: `false`
    pub display_units: bool,
    /// Display the axis ("X"/"Y") alongside the numbers.
    ///
    /// Default: `false`
    pub display_axis: bool,
    /// Display the physical pixel coordinates instead of the logical (with scaling factor applied).
    ///
    /// Default: `false`
    pub physical_pixels: bool,
    /// Display the value from the specified window (in a multi-window application).
    ///
    /// If `None` (the default), the primary window is selected.
    pub window: Option<Entity>,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Format the value string so that there is always space for this many digits.
    ///
    /// Default: `8` (assuming common up to 4-digit resolutions, precision = 0)
    pub width: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryWindowResolution {
    fn default() -> Self {
        PerfUiEntryWindowResolution {
            label: String::new(),
            display_units: false,
            display_axis: false,
            physical_pixels: false,
            separator: "x",
            window: None,
            precision: 0,
            width: 8,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display the current coordinates of the mouse cursor.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryCursorPosition {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Separate the X and Y values by this string.
    ///
    /// Default: `", "`.
    pub separator: &'static str,
    /// Display the unit ("px") alongside the numbers.
    ///
    /// Default: `false`
    pub display_units: bool,
    /// Display the axis ("X"/"Y") alongside the numbers.
    ///
    /// Default: `true`
    pub display_axis: bool,
    /// Display the physical pixel coordinates instead of the logical (with scaling factor applied).
    ///
    /// Default: `false`
    pub physical_pixels: bool,
    /// Display the value from the specified window (in a multi-window application).
    ///
    /// If `None` (the default), the primary window is selected.
    pub window: Option<Entity>,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Format the value string so that there is always space for this many digits.
    ///
    /// Default: `8` (assuming common up to 4-digit resolutions, precision = 0)
    pub width: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryCursorPosition {
    fn default() -> Self {
        PerfUiEntryCursorPosition {
            label: String::new(),
            display_units: false,
            display_axis: true,
            physical_pixels: false,
            separator: ", ",
            window: None,
            precision: 0,
            width: 8,
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryWindowMode {
    type Value = WindowMode;
    type SystemParam = (
        SQuery<&'static Window, With<PrimaryWindow>>,
        SQuery<&'static Window>,
    );

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Window Mode"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        (q_primary, q_any): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        if let Some(e) = self.window {
            Some(q_any.get(e).ok()?.mode)
        } else {
            Some(q_primary.single().ok()?.mode)
        }
    }
}

impl PerfUiEntry for PerfUiEntryWindowPresentMode {
    type Value = PresentMode;
    type SystemParam = (
        SQuery<&'static Window, With<PrimaryWindow>>,
        SQuery<&'static Window>,
    );

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Present Mode"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        (q_primary, q_any): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        if let Some(e) = self.window {
            Some(q_any.get(e).ok()?.present_mode)
        } else {
            Some(q_primary.single().ok()?.present_mode)
        }
    }
}

impl PerfUiEntry for PerfUiEntryWindowScaleFactor {
    type Value = f32;
    type SystemParam = (
        SQuery<&'static Window, With<PrimaryWindow>>,
        SQuery<&'static Window>,
    );

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Scale Factor"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        (q_primary, q_any): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        if let Some(e) = self.window {
            q_any.get(e).ok().map(|w| w.scale_factor())
        } else {
            q_primary.single().ok().map(|w| w.scale_factor())
        }
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value as f64)
    }
}

impl PerfUiEntry for PerfUiEntryWindowResolution {
    type Value = Vec2;
    type SystemParam = (
        SQuery<&'static Window, With<PrimaryWindow>>,
        SQuery<&'static Window>,
    );

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Resolution"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        (q_primary, q_any): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        if let Some(e) = self.window {
            if self.physical_pixels {
                q_any.get(e).ok().map(|w| Vec2::new(
                    w.physical_width() as f32,
                    w.physical_height() as f32,
                ))
            } else {
                q_any.get(e).ok().map(|w| Vec2::new(
                    w.width(),
                    w.height(),
                ))
            }
        } else {
            if self.physical_pixels {
                q_primary.single().ok().map(|w| Vec2::new(
                    w.physical_width() as f32,
                    w.physical_height() as f32,
                ))
            } else {
                q_primary.single().ok().map(|w| Vec2::new(
                    w.width(),
                    w.height(),
                ))
            }
        }
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        match (self.display_axis, self.display_units) {
            (true, true) => format!(
                "X: {:.p$} px{}Y: {:.p$} px",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
            (true, false) => format!(
                "X: {:.p$}{}Y: {:.p$}",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
            (false, true) => format!(
                "{:.p$} px{}{:.p$} px",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
            (false, false) => format!(
                "{:.p$}{}{:.p$}",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
        }
    }
}

impl PerfUiEntry for PerfUiEntryCursorPosition {
    type Value = Vec2;
    type SystemParam = (
        SQuery<&'static Window, With<PrimaryWindow>>,
        SQuery<&'static Window>,
    );

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Cursor Position"
        } else {
            &self.label
        }
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
    fn update_value(
        &self,
        (q_primary, q_any): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        if let Some(e) = self.window {
            if self.physical_pixels {
                q_any.get(e).ok()?.physical_cursor_position()
            } else {
                q_any.get(e).ok()?.cursor_position()
            }
        } else {
            if self.physical_pixels {
                q_primary.single().ok()?.physical_cursor_position()
            } else {
                q_primary.single().ok()?.cursor_position()
            }
        }
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        match (self.display_axis, self.display_units) {
            (true, true) => format!(
                "X: {:.p$} px{}Y: {:.p$} px",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
            (true, false) => format!(
                "X: {:.p$}{}Y: {:.p$}",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
            (false, true) => format!(
                "{:.p$} px{}{:.p$} px",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
            (false, false) => format!(
                "{:.p$}{}{:.p$}",
                value.x, self.separator, value.y, p = self.precision as usize
            ),
        }
    }
}
