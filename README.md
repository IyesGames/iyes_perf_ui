# Customizable Performance/Debug Overlay for Bevy UI

[![Crates.io](https://img.shields.io/crates/v/iyes_perf_ui)](https://crates.io/crates/iyes_perf_ui)
[![docs](https://docs.rs/iyes_perf_ui/badge.svg)](https://docs.rs/iyes_perf_ui/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)

Sponsor me:

<a href="https://github.com/sponsors/inodentry"><button class="ghsponsors-button">GitHub Sponsors</button></a>

Bevy Compatibility:

| Bevy Version | Plugin Version |
|--------------|----------------|
| `0.14`       | `0.3`          |
| `0.13`       | `0.2`,`0.1`    |

---

This crate provides an implementation of an in-game performance/debug UI overlay
for the [Bevy game engine](https://bevyengine.org).

The goal of this crate is to make it as useful as possible for any Bevy project:
 - Made with Bevy UI (not egui or any other 3rd-party UI solution)
 - Easy to set up (see [`simple`](examples/simple.rs) example)
 - Modular! You decide what info you want to display!
   - Choose any combination of predefined entries (see [`specific_entries`](examples/specific_entries.rs) example):
     - Framerate (FPS), Frame Time, Frame Count, ECS Entity Count, CPU Usage, RAM Usage,
       Wall Clock, Running Time, Fixed Time Step, Fixed Overstep,
       Cursor Position, Window Resolution, Window Scale Factor, Window Mode, Present Mode
   - Implement your own custom entries to display anything you like!
     - (see [`custom_minimal`](examples/custom_minimal.rs) and [`custom`](examples/custom.rs) examples)
 - Customizable appearance/styling (see [`settings`](examples/settings.rs), [`fps_minimalist`](examples/fps_minimalist.rs) examples)
 - Support for highlighting values using a custom font or color!
   - Allows you to quickly notice if something demands your attention.

Spawning a Perf UI can be as simple as:

```rust
commands.spawn(PerfUiBundle::default());
```

This creates a Perf UI with a curated selection of entries, which are in
my opinion the most useful out of everything provided in this crate.

If you want a UI with all the available entries (not recommended due
to performance overhead):

```rust
commands.spawn(PerfUiCompleteBundle::default());
```

If you want to create a Perf UI with specific entries of your choice,
just spawn an entity with `PerfUiRoot` + your desired entries, instead
of using the above bundles.

```rust
commands.spawn((
   PerfUiRoot::default(),
   PerfUiEntryFPS::default(),
   PerfUiEntryClock::default(),
   // ...
));
```

There are also some bundles to help you add some common groups of entries:

```rust
commands.spawn((
   PerfUiRoot::default(),
   // Contains everything related to FPS and frame time
   PerfUiFramerateEntries::default(),
   // Contains everything related to the window and cursor
   PerfUiWindowEntries::default(),
   // Contains everything related to system diagnostics (CPU, RAM)
   PerfUiSystemEntries::default(),
   // Contains everything related to fixed timestep
   PerfUiFixedTimeEntries::default(),
   // ...
));
```

If you want to customize the appearance, set the various fields in each of the
structs, instead of using `default()`.

![Screenshot of the simple example showing default configuration](screenshots/simple.png)

![Screenshot of the settings example showing multiple UIs with custom configuration](screenshots/settings.png)

## Fancy Widgets

It is possible to visualize the value in other ways, not just display it
as text.

`iyes_perf_ui` currently provides one such widget implementation: Bar. To
use it, wrap your entries in `PerfUiWidgetBar`.

For example, to display FPS as a Bar:

```rust
commands.spawn((
   PerfUiRoot::default(),
   PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
   // ...
));
```

If you want to create your own custom widgets, have a look at implementing
the `PerfUiWidget` trait.

## Performance Warning!

This crate is somewhere in-between "a useful diagnostic/dev tool" and "a tech demo".

Unfortunately, it does introduce significant overhead to your app, especially if you
spawn a "complete" UI with lots of entries/widgets.

Please keep this in mind: your game will run faster when you don't have a Perf UI spawned.
Silver lining: If your performance seems good with the Perf UI, it will be even better
without. ;)

To make it more representative of your actual performance, consider spawning a more
minimal Perf UI with just a few entries that are most useful to you (for example: fps,
frame time), instead of a "complete" UI.

---

I know it is ironic that a tool intended to help you measure your performance
ends up significantly degrading your performance. I am thinking about ways
to reduce the overhead.

From my own measurements, most of the overhead comes from `bevy_ui`'s layout
system struggling to update the complex layout of the Perf UI, not from any
of the actual code in `iyes_perf_ui`. So, to improve perfomance, I will need
to come up with a way to simplify the UI and make it easier for Bevy to process.
Or Bevy will have to get better at UI layout. ;)
