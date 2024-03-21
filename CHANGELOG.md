# Changelog

Notable user-facing changes with each release version will be described in this file.

## [0.2.1]: 2024-03-21

Added:
 - `ColorGradient::single()` constructor for when you want a fixed color.

## [0.2.0]: 2024-03-21

Added:
 - `PerfUiRoot.inner_background_color_highlight`: different background color for highlighted entries.
 - `PerfUiRoot.display_labels`: can be used to disable the labels column and show bare values.
 - `PerfUiRoot.layout_horizontal`: display entries horizontally instead of vertically.
 - [`fps_minimalist`](examples/fps_minimalist.rs) example, shows how to make a minimal FPS counter.
 - `label` field on all built-in entry types, to allow customizable label strings.
 - Built-in entries can now display units (opt-out) with their values, where relevant.
 - `PerfUiEntryCursorPosition` built-in entry.
 - `PerfUiEntryWindowResolution` built-in entry.
 - `PerfUiEntryWindowMode` built-in entry.
 - `PerfUiEntryWindowPresentMode` built-in entry.
 - `PerfUiEntryFixedTimeStep` built-in entry.
 - `PerfUiEntryFixedOverstep` built-in entry.
 - `PerfUiEntry::width_hint` optional method. If implemented, allows ensuring the UI is correctly
   sized and the values are correctly aligned at all times.
 - `utils::width_hint_pretty_*` helper functions for values formatted using `utils::format_pretty_*`.
 - `utils::ColorGradient` helper to allow for custom gradients / color interpolation.

Changed:
 - `enable_{color,highlight}` and `threshold_*` fields on built-in entry types that support
   coloring/highlighting based on value, have been replaced with `color_gradient` to allow full customization.
 - `PerfUiEntry::update_value` now takes `&self`. It is no longer allowed to mutate the component.
 - `PerfUiEntry::label` now returns `&str` instead of `String`.

Fixed:
 - The UI will now correctly update itself if you change any parameters after it has already been spawned.
 - UI entries will now correctly disappear if you remove their PerfUIEntry component.

Removed:
 - `utils::ryg_gradient_{up,down}`. Use the new `utils::ColorGradient` instead.

## [0.1.0]: 2024-03-19

Initial Release

[0.2.1]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.2.1
[0.2.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.2.0
[0.1.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.1.0
