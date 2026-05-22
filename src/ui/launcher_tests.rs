use crate::ui::launcher::{LauncherState, LauncherItemKind};
use winit::keyboard::KeyCode;
use winit::event::ElementState;

#[test]
fn test_launcher_initialization() {
    let state = LauncherState::new();
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.scroll_offset, 0);
    assert!(state.search_text.is_empty());
}

#[test]
fn test_launcher_filtering() {
    let mut state = LauncherState::new();
    
    // No search text: all items should be returned
    let filtered_all = state.filtered_items();
    assert_eq!(filtered_all.len(), state.items.len());
    
    // Search for "Snake"
    state.search_text = "Snake".to_string();
    let filtered_snake = state.filtered_items();
    assert_eq!(filtered_snake.len(), 1);
    assert_eq!(filtered_snake[0].1.name, "Snake Game");
    
    // Search for something that doesn't exist
    state.search_text = "NonExistent".to_string();
    let filtered_none = state.filtered_items();
    assert_eq!(filtered_none.len(), 0);
}

#[test]
fn test_launcher_navigation() {
    let mut state = LauncherState::new();
    let total_items = state.items.len();
    
    // Move down
    state.move_selection(1);
    assert_eq!(state.selected_index, 1);
    
    // Move up
    state.move_selection(-1);
    assert_eq!(state.selected_index, 0);
    
    // Boundary: top
    state.move_selection(-1);
    assert_eq!(state.selected_index, 0);
    
    // Boundary: bottom
    for _ in 0..total_items {
        state.move_selection(1);
    }
    assert_eq!(state.selected_index, total_items - 1);
}

#[test]
fn test_launcher_scrolling() {
    let mut state = LauncherState::new();
    
    // Initial state: scroll_offset = 0
    assert_eq!(state.scroll_offset, 0);
    
    // Scroll down (delta_y < 0)
    state.handle_mouse_wheel(0.0, -1.0);
    assert_eq!(state.scroll_offset, 1);
    
    // Scroll up (delta_y > 0)
    state.handle_mouse_wheel(0.0, 1.0);
    assert_eq!(state.scroll_offset, 0);
    
    // Boundary: top
    state.handle_mouse_wheel(0.0, 1.0);
    assert_eq!(state.scroll_offset, 0);
    
    // Scroll to bottom
    let total_items = state.items.len();
    for _ in 0..total_items {
        state.handle_mouse_wheel(0.0, -1.0);
    }
    // VISIBLE_ITEMS = 8
    assert_eq!(state.scroll_offset, total_items.saturating_sub(8));
}

#[test]
fn test_selection_scroll_sync() {
    let mut state = LauncherState::new();
    
    // Select item 10 (out of view)
    state.selected_index = 10;
    
    // Navigation should update scroll_offset
    state.move_selection(1); 
    
    assert_eq!(state.selected_index, 11);
    assert_eq!(state.scroll_offset, 11 - 8 + 1);
}

#[test]
fn test_search_resets_state() {
    let mut state = LauncherState::new();
    state.selected_index = 5;
    state.scroll_offset = 2;
    
    state.input_char('a');
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.scroll_offset, 0);
}
