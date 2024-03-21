//! Customizable Performance/Debug Overlay for Bevy UI
//!
//! This crate provides an implementation of an in-game performance/debug UI overlay
//! for the [Bevy game engine](https://bevyengine.org).
//!
//! The goal of this crate is to make it as useful as possible for any Bevy project:
//!  - Made with Bevy UI (not egui or any other 3rd-party UI solution)
//!  - Easy to set up (see `simple` example)
//!  - Customizable appearance/styling (see `settings` example)
//!  - Modular! You decide what info you want to display!
//!    - Choose any combination of predefined entries for:
//!      - Framerate (FPS), Frame Time, Frame Count, ECS Entity Count, CPU Usage, RAM Usage, Wall Clock, Running Time
//!    - Implement your own custom entries to display anything you like!
//!      - (see `custom_minimal` and `custom` examples)
//!  - Support for highlighting values using a custom font or color!
//!    - Allows you to quickly notice if something demands your attention.

#![warn(missing_docs)]

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::ecs::system::{StaticSystemParam, SystemParam};

/// Prelude of common types for users of the library
pub mod prelude {
    pub use crate::{
        PerfUiPlugin,
        PerfUiAppExt,
        PerfUiRoot,
        PerfUiEntry,
        PerfUiPosition,
    };
    pub use crate::diagnostics::{
        PerfUiEntryFPS,
        PerfUiEntryFrameTime,
        PerfUiEntryFPSWorst,
        PerfUiEntryFrameTimeWorst,
        PerfUiEntryFrameCount,
        PerfUiEntryEntityCount,
        PerfUiEntryCpuUsage,
        PerfUiEntryMemUsage,
    };
    pub use crate::time::{
        PerfUiEntryClock,
        PerfUiEntryRunningTime,
        PerfUiEntryFixedTimeStep,
        PerfUiEntryFixedOverstep,
    };
    pub use crate::window::{
        PerfUiEntryWindowResolution,
        PerfUiEntryWindowMode,
        PerfUiEntryWindowPresentMode,
        PerfUiEntryCursorPosition,
    };
}

pub mod utils;

pub mod diagnostics;
pub mod time;
pub mod window;

/// The Bevy Plugin
#[derive(Default)]
pub struct PerfUiPlugin;

impl Plugin for PerfUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            setup_perf_ui
                .run_if(rc_setup_perf_ui)
                .in_set(PerfUiSet::Setup),
            sort_perf_ui_entries
                .run_if(rc_sort_perf_ui_entries)
                .after(PerfUiSet::Setup),
        ).run_if(any_with_component::<PerfUiRoot>));
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryFPS>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryFrameTime>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryFPSWorst>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryFrameTimeWorst>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryFrameCount>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryEntityCount>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryCpuUsage>();
        app.add_perf_ui_entry_type::<crate::diagnostics::PerfUiEntryMemUsage>();
        app.add_perf_ui_entry_type::<crate::time::PerfUiEntryClock>();
        app.add_perf_ui_entry_type::<crate::time::PerfUiEntryRunningTime>();
        app.add_perf_ui_entry_type::<crate::time::PerfUiEntryFixedTimeStep>();
        app.add_perf_ui_entry_type::<crate::time::PerfUiEntryFixedOverstep>();
        app.add_perf_ui_entry_type::<crate::window::PerfUiEntryWindowResolution>();
        app.add_perf_ui_entry_type::<crate::window::PerfUiEntryWindowMode>();
        app.add_perf_ui_entry_type::<crate::window::PerfUiEntryWindowPresentMode>();
        app.add_perf_ui_entry_type::<crate::window::PerfUiEntryCursorPosition>();
    }
}

/// Extension trait for adding new types of Perf UI Entries.
pub trait PerfUiAppExt {
    /// Add support for a custom perf UI entry type (component).
    fn add_perf_ui_entry_type<T: PerfUiEntry>(&mut self) -> &mut Self;
}

impl PerfUiAppExt for App {
    fn add_perf_ui_entry_type<T: PerfUiEntry>(&mut self) -> &mut Self {
        self.add_systems(Update, (
            setup_perf_ui_entry::<T>
                .run_if(rc_setup_perf_ui_entry::<T>)
                .after(setup_perf_ui)
                .in_set(PerfUiSet::Setup),
            update_perf_ui_entry::<T>
                .run_if(any_with_component::<PerfUiEntryMarker<T>>)
                .after(setup_perf_ui_entry::<T>)
                .in_set(PerfUiSet::Update),
        ));
        self
    }
}

/// System Set to allow you to order things relative to our systems.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerfUiSet {
    /// Systems that manage (spawn) the Perf UI entity hierarchy.
    Setup,
    /// Systems that update the values of Perf UI entries (of any type).
    ///
    /// If you care about a specific entry only, refer to the `update_perf_ui_entry::<T>` system instead.
    Update,
}

/// Trait for components representing entries (rows) in the Perf UI.
///
/// If you want to display your own info in Perf UI, create your
/// own component types and implement this trait for them.
pub trait PerfUiEntry: Component {
    /// Any system parameters you need to fetch/update the value.
    type SystemParam: SystemParam + 'static;

    /// The raw (unformatted) type of the value to be displayed.
    type Value: std::fmt::Debug;

    /// The label text to display in the Perf UI.
    fn label(&self) -> &str;

    /// Optional: provide a desired width for the value string.
    ///
    /// The formatted value will be padded with spaces. This allows
    /// everything to line up nicely in the UI and prevents the UI from
    /// spontaneously resizing as the values change.
    ///
    /// (assuming a monospace font)
    fn width_hint(&self) -> usize {
        0
    }

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
    /// Called every frame after `update_value`, unless it returned `None`.
    /// The `value` parameter is whatever that function returned.
    fn value_color(
        &self,
        _value: &Self::Value,
    ) -> Option<Color> {
        None
    }

    /// Optional: set whether the value should be displayed using the "highlighted" font.
    ///
    /// Called every frame after `update_value`, unless it returned `None`.
    /// The `value` parameter is whatever that function returned.
    fn value_highlight(
        &self,
        _value: &Self::Value,
    ) -> bool {
        false
    }

    /// The sort key controls where the entry will appear in the Perf UI.
    fn sort_key(&self) -> i32;
}

/// Which corner of the screen to display the Perf UI at?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerfUiPosition {
    /// Absolute positioning based on distance from top and left edges of viewport.
    TopLeft,
    /// Absolute positioning based on distance from top and right edges of viewport.
    #[default]
    TopRight,
    /// Absolute positioning based on distance from bottom and left edges of viewport.
    BottomLeft,
    /// Absolute positioning based on distance from bottom and right edges of viewport.
    BottomRight,
}

/// Component to configure a Perf UI instance.
///
/// To create a Perf UI, spawn an entity with this component
/// + any components for the entries you want to display:
///
/// ```rust
/// commands.spawn((
///     PerfUiRoot {
///         // ... settings ...
///         ..default()
///     },
///     PerfUiEntryFPS {
///         // ... settings ...
///         ..default()
///     },
///     /// ...
/// ));
/// ```
///
/// We will automatically detect that you have added these components
/// and will do the rest of the setup to spawn the UI. :)
#[derive(Component, Debug, Clone)]
pub struct PerfUiRoot {
    /// The color to use for the background of the Perf UI.
    ///
    /// Default: BLACK with alpha 0.5
    pub background_color: Color,
    /// The color to use for the background of each entry/row.
    ///
    /// Default: NONE
    pub inner_background_color: Color,
    /// The color to use for the background of highlighted entries.
    ///
    /// Default: RED with alpha 1/16
    pub inner_background_color_highlight: Color,
    /// Should labels be displayed?
    /// If false, there will be no column for labels, only bare values.
    ///
    /// Default: `true`
    pub display_labels: bool,
    /// The text to display if a value cannot be obtained.
    ///
    /// Default: `"N/A"`
    pub text_err: String,
    /// The color for the error text.
    ///
    /// Default: DARK_GRAY
    pub err_color: Color,
    /// The color to use for entries that do not provide a custom color.
    ///
    /// Default: GRAY
    pub default_value_color: Color,
    /// The color to use for label text.
    ///
    /// Default: WHITE
    pub label_color: Color,
    /// The font to use for labels.
    pub font_label: Handle<Font>,
    /// The font to use for values.
    pub font_value: Handle<Font>,
    /// The font to use for highlighted values.
    pub font_highlight: Handle<Font>,
    /// The font size for labels.
    ///
    /// Default: `16.0`
    pub fontsize_label: f32,
    /// The font size for values.
    ///
    /// Default: `18.0`
    pub fontsize_value: f32,
    /// The ZIndex of the UI.
    ///
    /// Default: `Global(i32::MAX)` (display on top of all other UI)
    pub z_index: ZIndex,
    /// The position of the UI.
    ///
    /// Default: top-right corner
    pub position: PerfUiPosition,
    /// Distance from the edge of the screen in pixels
    ///
    /// Default: `16.0`
    pub margin: f32,
    /// Empty space around the edge of the Perf UI
    ///
    /// Default: `2.0`
    pub padding: f32,
    /// Empty space around entries (rows) in pixels
    ///
    /// Default: `0.0`
    pub inner_margin: f32,
    /// Empty space around the text in every row
    ///
    /// Default: `0.0`
    pub inner_padding: f32,
    /// Force a fixed width (in pixels) for the values column
    ///
    /// Default: `None`
    pub values_col_width: Option<f32>,
}

impl Default for PerfUiRoot {
    fn default() -> Self {
        PerfUiRoot {
            background_color: Color::BLACK.with_a(0.5),
            inner_background_color: Color::NONE,
            inner_background_color_highlight: Color::RED.with_a(1.0 / 16.0),
            display_labels: true,
            text_err: "N/A".into(),
            err_color: Color::DARK_GRAY,
            default_value_color: Color::GRAY,
            label_color: Color::WHITE,
            font_label: default(),
            font_value: default(),
            font_highlight: default(),
            fontsize_label: 16.0,
            fontsize_value: 18.0,
            z_index: ZIndex::Global(i32::MAX),
            position: default(),
            margin: 16.0,
            padding: 2.0,
            inner_margin: 0.0,
            inner_padding: 0.0,
            values_col_width: None,
        }
    }
}

impl PerfUiPosition {
    fn top(self, margin: f32) -> Val {
        match self {
            PerfUiPosition::TopLeft | PerfUiPosition::TopRight => Val::Px(margin),
            PerfUiPosition::BottomLeft | PerfUiPosition::BottomRight => Val::Auto,
        }
    }
    fn bottom(self, margin: f32) -> Val {
        match self {
            PerfUiPosition::BottomLeft | PerfUiPosition::BottomRight => Val::Px(margin),
            PerfUiPosition::TopLeft | PerfUiPosition::TopRight => Val::Auto,
        }
    }
    fn left(self, margin: f32) -> Val {
        match self {
            PerfUiPosition::TopLeft | PerfUiPosition::BottomLeft => Val::Px(margin),
            PerfUiPosition::TopRight | PerfUiPosition::BottomRight => Val::Auto,
        }
    }
    fn right(self, margin: f32) -> Val {
        match self {
            PerfUiPosition::TopRight | PerfUiPosition::BottomRight => Val::Px(margin),
            PerfUiPosition::TopLeft | PerfUiPosition::BottomLeft => Val::Auto,
        }
    }
}

#[derive(Component)]
struct PerfUiEntryMarker<T: PerfUiEntry> {
    e_root: Entity,
    _pd: PhantomData<T>,
}

#[derive(Component)]
struct PerfUiTextMarker<T: PerfUiEntry> {
    e_root: Entity,
    e_entry: Entity,
    _pd: PhantomData<T>,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PerfUiSortKey(i32);

fn rc_setup_perf_ui(
    q: Query<(), Changed<PerfUiRoot>>,
) -> bool {
    !q.is_empty()
}

fn setup_perf_ui(
    mut commands: Commands,
    mut q_root: Query<(Entity, &PerfUiRoot, Option<&mut BackgroundColor>, Option<&mut Style>), Changed<PerfUiRoot>>,
) {
    for (e, perf_ui, background, style) in &mut q_root {
        let new_style = Style {
            position_type: PositionType::Absolute,
            top: perf_ui.position.top(perf_ui.margin),
            bottom: perf_ui.position.bottom(perf_ui.margin),
            left: perf_ui.position.left(perf_ui.margin),
            right: perf_ui.position.right(perf_ui.margin),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            padding: UiRect::all(Val::Px(perf_ui.padding)),
            ..default()
        };
        if let (Some(mut background), Some(mut style)) = (background, style) {
            background.0 = perf_ui.background_color;
            *style = new_style;
        } else {
            commands.entity(e).insert((
                NodeBundle {
                    background_color: BackgroundColor(perf_ui.background_color),
                    style: new_style,
                    ..default()
                },
            ));
        }
    }
}

fn rc_setup_perf_ui_entry<T: PerfUiEntry>(
    q: Query<(), Or<(Changed<T>, Changed<PerfUiRoot>)>>,
    removed: RemovedComponents<T>,
) -> bool {
    !q.is_empty() || !removed.is_empty()
}

fn setup_perf_ui_entry<T: PerfUiEntry>(
    mut commands: Commands,
    q_root: Query<(Entity, &PerfUiRoot, &T), Or<(Changed<T>, Changed<PerfUiRoot>)>>,
    q_entry: Query<(Entity, &PerfUiEntryMarker<T>)>,
    mut removed: RemovedComponents<T>,
) {
    // handle any removals:
    // if the entry component was removed from a perf ui root entity,
    // we need to find the entity of the entry's UI and despawn it.
    for e_removed in removed.read() {
        if let Some(e_entry) = q_entry.iter()
            .find(|(_, marker)| marker.e_root == e_removed)
            .map(|(e, _)| e)
        {
            commands.entity(e_removed)
                .remove_children(&[e_entry]);
            commands.entity(e_entry).despawn_recursive();
        }
    }
    // handle any additions or reconfigurations:
    // if an entry component was added/changed to a perf ui root entity,
    // or if the ui root component itself was changed,
    // find and despawn any existing entries and
    // spawn a new UI hierarchy for the entry.
    for (e_root, perf_ui, entry) in &q_root {
        // despawn any old/existing UI hierarchy for relevant entries
        if let Some(e_entry) = q_entry.iter()
            .find(|(_, marker)| marker.e_root == e_root)
            .map(|(e, _)| e)
        {
            commands.entity(e_root)
                .remove_children(&[e_entry]);
            commands.entity(e_entry).despawn_recursive();
        }

        // spawn the new UI hierarchy
        let e_entry = commands.spawn((
            PerfUiEntryMarker::<T> {
                e_root,
                _pd: PhantomData,
            },
            PerfUiSortKey(entry.sort_key()),
            NodeBundle {
                background_color: BackgroundColor(perf_ui.inner_background_color),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(perf_ui.inner_margin)),
                    padding: UiRect::all(Val::Px(perf_ui.inner_padding)),
                    ..default()
                },
                ..default()
            },
        )).id();
        if perf_ui.display_labels {
            let e_label_wrapper = commands.spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
            )).id();
            let e_label = commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("{}: ", entry.label()),
                        TextStyle {
                            font: perf_ui.font_label.clone(),
                            font_size: perf_ui.fontsize_label,
                            color: perf_ui.label_color,
                        }
                    ),
                    ..default()
                },
            )).id();
            commands.entity(e_label_wrapper).push_children(&[e_label]);
            commands.entity(e_entry).push_children(&[e_label_wrapper]);
        }
        let e_text_wrapper = commands.spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(4.0)),
                    width: if let Some(w) = perf_ui.values_col_width {
                        Val::Px(w)
                    } else {
                        Val::Auto
                    },
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            },
        )).id();
        let e_text = commands.spawn((
            PerfUiTextMarker::<T> {
                e_root,
                e_entry,
                _pd: PhantomData,
            },
            TextBundle {
                text: Text::from_section(
                    perf_ui.text_err.clone(),
                    TextStyle {
                        font: perf_ui.font_value.clone(),
                        font_size: perf_ui.fontsize_label,
                        color: perf_ui.err_color,
                    }
                ),
                ..default()
            },
        )).id();
        commands.entity(e_text_wrapper).push_children(&[e_text]);
        commands.entity(e_entry).push_children(&[e_text_wrapper]);
        commands.entity(e_root).push_children(&[e_entry]);
    }
}

fn rc_sort_perf_ui_entries(
    q: Query<(), (With<PerfUiRoot>, Changed<Children>)>,
) -> bool {
    !q.is_empty()
}

fn sort_perf_ui_entries(
    mut q_root: Query<&mut Children, (With<PerfUiRoot>, Changed<Children>)>,
    q_sortkey: Query<&PerfUiSortKey>,
) {
    for mut children in &mut q_root {
        children.sort_by_key(|e| q_sortkey.get(*e).map(|k| k.0).unwrap_or(0));
    }
}

/// System that updates the values of Perf UI entries of a given type
///
/// Exposed as `pub` so you can refer to it for ordering.
#[allow(private_interfaces)]
pub fn update_perf_ui_entry<T: PerfUiEntry>(
    q_root: Query<(&PerfUiRoot, &T)>,
    mut q_entry: Query<&mut BackgroundColor, With<PerfUiEntryMarker<T>>>,
    mut q_text: Query<(&mut Text, &PerfUiTextMarker<T>)>,
    entry_param: StaticSystemParam<T::SystemParam>,
) {
    let mut entry_param = entry_param.into_inner();
    for (mut text, marker) in &mut q_text {
        let Ok((root, entry)) = q_root.get(marker.e_root) else {
            continue;
        };
        let mut entry_highlight = false;
        if let Some(value) = entry.update_value(&mut entry_param) {
            let color = entry.value_color(&value)
                .unwrap_or(root.default_value_color);
            let s = entry.format_value(&value);
            let width_hint = entry.width_hint();
            text.sections[0].value = if s.len() < width_hint {
                format!("{:>w$}", s, w = width_hint)
            } else {
                s
            };
            text.sections[0].style.color = color;
            if entry.value_highlight(&value) {
                text.sections[0].style.font = root.font_highlight.clone();
                entry_highlight = true;
            } else {
                text.sections[0].style.font = root.font_value.clone();
            }
        } else {
            let s = root.text_err.clone();
            let width_hint = entry.width_hint();
            text.sections[0].value = if s.len() < width_hint {
                format!("{:>w$}", s, w = width_hint)
            } else {
                s
            };
            text.sections[0].style.color = root.err_color;
            text.sections[0].style.font = root.font_value.clone();
        }
        if let Ok(mut entry_bgcolor) = q_entry.get_mut(marker.e_entry) {
            if entry_highlight {
                entry_bgcolor.0 = root.inner_background_color_highlight;
            } else {
                entry_bgcolor.0 = root.inner_background_color;
            }
        }
    }
}
