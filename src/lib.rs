//! Customizable Performance/Debug Overlay for Bevy UI
//!
//! This crate provides an implementation of an in-game performance/debug UI overlay
//! for the [Bevy game engine](https://bevyengine.org).
//!
//! The goal of this crate is to make it as useful as possible for any Bevy project:
//!  - Made with Bevy UI (not egui or any other 3rd-party UI solution)
//!  - Easy to set up (see [`simple`](https://github.com/IyesGames/iyes_perf_ui/blob/v0.2.3/examples/simple.rs) example)
//!  - Modular! You decide what info you want to display!
//!    - Choose any combination of predefined entries
//!      (see [`specific_entries`](https://github.com/IyesGames/iyes_perf_ui/blob/v0.2.3/examples/specific_entries.rs) example):
//!      - Framerate (FPS), Frame Time, Frame Count, ECS Entity Count, CPU Usage, RAM Usage,
//!        Render CPU Time, Render GPU Time,
//!        Wall Clock, Running Time, Fixed Time Step, Fixed Overstep,
//!        Cursor Position, Window Resolution, Window Scale Factor, Window Mode, Present Mode
//!    - Implement your own custom entries to display anything you like!
//!      - (see [`custom_minimal`](https://github.com/IyesGames/iyes_perf_ui/blob/v0.2.3/examples/custom_minimal.rs) and [`custom`](https://github.com/IyesGames/iyes_perf_ui/blob/v0.2.3/examples/custom.rs) examples)
//!  - Customizable appearance/styling (see [`settings`](https://github.com/IyesGames/iyes_perf_ui/blob/v0.2.3/examples/settings.rs), [`fps_minimalist`](https://github.com/IyesGames/iyes_perf_ui/blob/v0.2.3/examples/fps_minimalist.rs) examples)
//!  - Support for highlighting values using a custom font or color!
//!    - Allows you to quickly notice if something demands your attention.
//!
//! ---
//!
//! First, make sure to add the plugin to your app:
//!
//! ```rust
//! app.add_plugins(PerfUiPlugin);
//! ```
//!
//! And then, pawning a Perf UI can be as simple as:
//!
//! ```rust
//! commands.spawn(PerfUiAllEntries::default());
//! ```
//!
//! If you want to create a Perf UI with specific entries of your choice,
//! just spawn an entity with your desired entries, instead
//! of using this bundle.
//!
//! ```rust
//! commands.spawn((
//!     PerfUiEntryFPS::default(),
//!     PerfUiEntryClock::default(),
//!     // ...
//! ));
//! ```
//!
//! If you want to customize the appearance, set the various fields in each of the
//! structs, instead of using `default()`. To customize settings that apply to all
//! entries, add the [`PerfUiRoot`] component.
//!
//! If you want to implement your own custom entry, create a component type
//! to represent your entry (you can use it to store any settings),
//! implement [`PerfUiEntry`] for it, and register it using
//! `app.add_perf_ui_entry_type::<T>()`.

#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![allow(clippy::collapsible_else_if)]

use bevy::prelude::*;

/// Prelude of common types for users of the library
pub mod prelude {
    pub use crate::{
        PerfUiPlugin,
        PerfUiAppExt,
    };
    pub use crate::ui::root::{
        PerfUiRoot,
        PerfUiPosition,
    };
    pub use crate::utils::ColorGradient;
    #[cfg(feature = "entries")]
    pub use crate::entries::prelude::*;
    #[cfg(feature = "widgets")]
    pub use crate::widgets::prelude::*;
}

pub mod entry;
pub mod ui;
pub mod utils;

#[cfg(feature = "entries")]
pub mod entries;
#[cfg(feature = "widgets")]
pub mod widgets;

/// The Bevy Plugin
#[derive(Default)]
pub struct PerfUiPlugin;

impl Plugin for PerfUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            crate::ui::root::setup_perf_ui
                .run_if(crate::ui::root::rc_setup_perf_ui)
                .in_set(PerfUiSet::Setup),
            crate::ui::sort_perf_ui_widgets
                .run_if(crate::ui::rc_sort_perf_ui_widgets)
                .after(PerfUiSet::Setup),
        )
            .run_if(crate::ui::rc_any_visible)
        );

        #[cfg(feature = "entries")]
        app.add_plugins(entries::predefined_entries_plugin);
        #[cfg(all(feature = "entries", feature = "widgets"))]
        app.add_plugins(widgets::predefined_widgets_plugin);
    }
}

/// Extension trait for adding new types of Perf UI Entries.
pub trait PerfUiAppExt {
    /// Add support for a custom Perf UI Widget type (component).
    ///
    /// Widgets are paired to entry types. This method adds support
    /// for displaying a specific entry type using a specific widget.
    fn add_perf_ui_widget<W, E>(&mut self) -> &mut Self
    where
        E: crate::entry::PerfUiEntry,
        W: crate::ui::widget::PerfUiWidget<E>;

    /// Add support for a custom Perf UI Entry type (component).
    ///
    /// This adds support for displaying the provided entry type
    /// using the builtin "simple" widget, which just shows the
    /// label string and the current value.
    ///
    /// If you want to display your data in other ways, consider
    /// also calling `add_perf_ui_widget` to add support for displaying
    /// your entry using different UI widgets.
    fn add_perf_ui_simple_entry<T: crate::entry::PerfUiEntry>(&mut self) -> &mut Self {
        self.add_perf_ui_widget::<T, T>();
        self
    }
}

impl PerfUiAppExt for App {
    fn add_perf_ui_widget<W, E>(&mut self) -> &mut Self
    where
        E: crate::entry::PerfUiEntry,
        W: crate::ui::widget::PerfUiWidget<E>
    {
        self.add_systems(Update, (
            crate::ui::widget::setup_perf_ui_widget::<E, W>
                .run_if(crate::ui::widget::rc_setup_perf_ui_widget::<E, W>)
                .after(crate::ui::root::setup_perf_ui)
                .in_set(PerfUiSet::Setup),
            crate::ui::widget::update_perf_ui_widget::<E, W>
                .run_if(any_with_component::<crate::ui::widget::PerfUiWidgetMarker<W>>)
                .after(crate::ui::widget::setup_perf_ui_widget::<E, W>)
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
    /// If you care about a specific entry only, refer to the `update_perf_ui_widget::<T>` system instead.
    Update,
}
