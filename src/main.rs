use bevy::prelude::*;

fn main() {
    App::new() 
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_cursor_input, handle_mouse_hover, update_cursor_position))
        .add_plugins(DefaultPlugins)
        .run();
}

// Components - pure data, no behavior
#[derive(Component)]
struct MenuCursor {
    selected_index: usize,
}

#[derive(Component)]
struct MenuItem {
    index: usize,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
    // Spawn title text - moved up to y = 200.0
    commands.spawn((
        Text2d::new("Sudoku"),
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
    ));
    
    // Spawn menu items
    let menu_items = vec!["Start Game", "Quit"];
    let start_y = 50.0;
    let spacing = 50.0;
    
    for (index, item_text) in menu_items.iter().enumerate() {
        let y_pos = start_y - (index as f32 * spacing);
        commands.spawn((
            Text2d::new(*item_text),
            Transform::from_translation(Vec3::new(0.0, y_pos, 0.0)),
            MenuItem { index },
        ));
    }
    
    // Spawn cursor (the ">" indicator)
    commands.spawn((
        Text2d::new(">"),
        Transform::from_translation(Vec3::new(-100.0, start_y, 0.0)),
        MenuCursor { selected_index: 0 },
    ));
}

// System: Handle keyboard input to move cursor
fn handle_cursor_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cursor_query: Query<&mut MenuCursor>,
) {
    // Query the cursor component (only one cursor exists)
    if let Ok(mut cursor) = cursor_query.single_mut() {
        let max_index = 1; // We have 2 menu items (0, 1)
        
        // Move cursor down (increase index)
        if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
            cursor.selected_index = (cursor.selected_index + 1).min(max_index);
        }
        
        // Move cursor up (decrease index)
        if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
            cursor.selected_index = cursor.selected_index.saturating_sub(1);
        }
    }
}

// System: Handle mouse hover to select menu items
fn handle_mouse_hover(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    menu_items_query: Query<(&MenuItem, &Transform)>,
    mut cursor_query: Query<&mut MenuCursor>,
) {
    // Get the primary window and camera
    // Using Result type annotations for clarity
    let window: &Window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    
    let (camera, camera_transform): (&Camera, &GlobalTransform) = match camera_query.single() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    // Get the cursor position in the window
    let Some(cursor_pos) = window.cursor_position() else {
        return; // Cursor not in window
    };
    
    // Convert screen coordinates to world coordinates
    // In Bevy 0.17, viewport_to_world_2d returns a Result
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };
    
    // Check which menu item is being hovered over
    // We'll use a simple distance check (items are about 50 pixels apart vertically)
    let hover_threshold = 25.0; // Half the spacing between items
    
    for (menu_item, item_transform) in menu_items_query.iter() {
        let item_pos = item_transform.translation.truncate(); // Convert Vec3 to Vec2
        let distance = world_pos.distance(item_pos);
        
        // If mouse is close to this item, select it
        if distance < hover_threshold {
            if let Ok(mut cursor) = cursor_query.single_mut() {
                cursor.selected_index = menu_item.index;
            }
            break;
        }
    }
}

// System: Update the cursor's visual position based on selected index
fn update_cursor_position(
    mut cursor_query: Query<(&MenuCursor, &mut Transform), Changed<MenuCursor>>,
    menu_items_query: Query<(&MenuItem, &Transform), Without<MenuCursor>>,
) {
    // Only run if the cursor component changed (optimization with Changed<>)
    // Combining MenuCursor and Transform in one query avoids the query conflict
    if let Ok((cursor, mut cursor_transform)) = cursor_query.single_mut() {
        // Find the menu item matching the selected index
        for (menu_item, item_transform) in menu_items_query.iter() {
            if menu_item.index == cursor.selected_index {
                // Update cursor's Y position to match the selected menu item
                cursor_transform.translation.y = item_transform.translation.y;
                break;
            }
        }
    }
}