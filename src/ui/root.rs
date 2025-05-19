//! The Root of the Perf UI.
//!
//! This is where the properties for the whole Perf UI are set,
//! and what manages the UI for all your entries.

use bevy::prelude::*;

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
    /// Display entries horizontally instead of vertically.
    ///
    /// Default: `false`
    pub layout_horizontal: bool,
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
    /// Default: `12.0`
    pub fontsize_label: f32,
    /// The font size for values.
    ///
    /// Default: `12.0`
    pub fontsize_value: f32,
    /// The ZIndex of the UI.
    ///
    /// Default: `i32::MAX` (display on top of all other UI)
    pub z_index: GlobalZIndex,
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
    /// The width (in pixels) of the values column
    ///
    /// Default: `128.0`
    pub values_col_width: f32,
}

impl Default for PerfUiRoot {
    fn default() -> Self {
        PerfUiRoot {
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.5),
            inner_background_color: Color::NONE,
            inner_background_color_highlight: Color::srgba(1.0, 0.0, 0.0, 1.0 / 16.0),
            display_labels: true,
            layout_horizontal: false,
            text_err: "N/A".into(),
            err_color: Color::srgb(0.5, 0.5, 0.5),
            default_value_color: Color::srgb(0.75, 0.75, 0.75),
            label_color: Color::srgb(1.0, 1.0, 1.0),
            font_label: default(),
            font_value: default(),
            font_highlight: default(),
            fontsize_label: 12.0,
            fontsize_value: 12.0,
            z_index: GlobalZIndex(i32::MAX),
            position: default(),
            margin: 16.0,
            padding: 2.0,
            inner_margin: 0.0,
            inner_padding: 0.0,
            values_col_width: 128.0,
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

pub(crate) fn rc_setup_perf_ui(
    q: Query<(), Changed<PerfUiRoot>>,
) -> bool {
    !q.is_empty()
}

pub(crate) fn setup_perf_ui(
    mut commands: Commands,
    fonts: Res<Assets<Font>>,
    mut q_root: Query<(Entity, &PerfUiRoot, Option<&mut BackgroundColor>, Option<&mut Node>), Changed<PerfUiRoot>>,
) {
    for (e, perf_ui, background, style) in &mut q_root {
        if (perf_ui.font_label == Handle::default()
            || perf_ui.font_value == Handle::default()
            || perf_ui.font_highlight == Handle::default())
            && !fonts.contains(&Handle::default())
        {
            error!("Bevy's default font is missing. Either enable Bevy's `default_font` cargo feature, or specify custom fonts in `PerfUiRoot`.");
        }
        let new_style = Node {
            position_type: PositionType::Absolute,
            top: perf_ui.position.top(perf_ui.margin),
            bottom: perf_ui.position.bottom(perf_ui.margin),
            left: perf_ui.position.left(perf_ui.margin),
            right: perf_ui.position.right(perf_ui.margin),
            flex_direction: if perf_ui.layout_horizontal {
                FlexDirection::Row
            } else {
                FlexDirection::Column
            },
            align_items: AlignItems::Stretch,
            padding: UiRect::all(Val::Px(perf_ui.padding)),
            ..default()
        };
        if let (Some(mut background), Some(mut style)) = (background, style) {
            background.0 = perf_ui.background_color;
            *style = new_style;
        } else {
            commands.entity(e).insert((
                Name::new("PerfUi"),
                BackgroundColor(perf_ui.background_color),
                new_style
            ));
        }
    }
}
