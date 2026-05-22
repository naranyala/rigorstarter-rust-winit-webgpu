# Launcher

The `Launcher` is the central hub of the application, allowing users to quickly switch between different games and technical examples.

## How it Works

The `LauncherState` maintains a list of `LauncherItem` objects, each representing a state that can be pushed onto the `StateManager` stack.

## Key Features
- **Search**: A real-time filter allows users to find specific items by typing.
- **Navigation**: Supports both keyboard (Arrow keys, Enter) and mouse (clicks, scrolling) navigation.
- **Scrolling**: A virtualized list that only renders visible items, supporting any number of entries.

## Adding New Content

To add a new game or example:
1. Create a new state implementing the `State` trait in `src/game/` or `src/examples/`.
2. Register the module in `src/game/mod.rs` or `src/examples/mod.rs`.
3. Add a new `LauncherItem` to the `LauncherState::new()` list.
4. Update the `match` block in `handle_input` and `handle_mouse_click` to instantiate the new state.
