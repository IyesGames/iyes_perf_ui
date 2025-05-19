//! Common framework for Perf UI Entry types (data providers)

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[allow(unused_imports)]
use crate::prelude::*;

/// Trait for components representing entries (rows) in the Perf UI.
///
/// If you want to display your own info in Perf UI, create your
/// own component types and implement this trait for them.
///
/// If you also have meaningful historical data available, rather than
/// just the current value, consider also implementing [`PerfUiEntryHistory`].
pub trait PerfUiEntry: Component {
    /// Any system parameters you need to fetch/update the value.
    type SystemParam: SystemParam + 'static;

    /// The raw (unformatted) type of the value to be displayed.
    type Value: std::fmt::Debug;

    /// The label text to display in the Perf UI.
    fn label(&self) -> &str;

    /// The sort key controls where the entry will appear in the Perf UI.
    ///
    /// The recommended way to implement this is to have a field in your struct,
    /// which you can set to `iyes_perf_ui::utils::next_sort_key()` in your
    /// `impl Default`. Then return that value here.
    ///
    /// That way, the entry will be sorted according to the order in which the
    /// user creates the entries.
    fn sort_key(&self) -> i32;

    /// Update the value to display in the Perf UI.
    ///
    /// This function will be called once per frame,
    /// in the `Update` schedule,
    /// in the `PerfUiSet::Update` set.
    fn update_value(
        &self,
        param: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value>;

    /// Format the raw value into a string for display
    ///
    /// Called every frame after `update_value`, unless it returned `None`.
    /// The `value` parameter is whatever that function returned.
    ///
    /// If unimplemented, the value will be formatted with its `Debug` impl.
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format!("{:?}", value)
    }

    /// Optional: set a custom color for the value to display.
    ///
    /// `None` means the value should be displayed using the default color.
    ///
    /// Called every frame after `update_value`, unless it returned `None`.
    /// The `value` parameter is whatever that function returned.
    fn value_color(
        &self,
        _value: &Self::Value,
    ) -> Option<Color> {
        None
    }

    /// Optional: set whether the value should be displayed highlighted.
    ///
    /// Called every frame after `update_value`, unless it returned `None`.
    /// The `value` parameter is whatever that function returned.
    fn value_highlight(
        &self,
        _value: &Self::Value,
    ) -> bool {
        false
    }
}

/// Extension to [`PerfUiEntry`] to provide an expected range of values.
///
/// Used by widgets which need to visualize the value within a range,
/// such as [`PerfUiWidgetBar`].
pub trait PerfUiEntryDisplayRange: PerfUiEntry {
    /// Provide an upper bound for the value.
    ///
    /// This is used by some widgets to influence
    /// how to visualize the value.
    ///
    /// If the value is above this, it may be clipped in the UI.
    fn max_value_hint(&self) -> Option<Self::Value>;

    /// Provide a lower bound for the value
    ///
    /// This is used by some widgets to influence
    /// how to visualize the value.
    ///
    /// If the value is below this, it may be clipped in the UI.
    fn min_value_hint(&self) -> Option<Self::Value>;
}
