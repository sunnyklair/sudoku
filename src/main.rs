use bevy::prelude::*;

// Layout constants
const MENU_ITEMS: &[&str] = &["Start Game", "Quit"];
const TITLE_Y: f32 = 200.0;
const MENU_START_Y: f32 = 50.0;
const MENU_SPACING: f32 = 50.0;
const CURSOR_X_OFFSET: f32 = -100.0;
const HOVER_THRESHOLD: f32 = 25.0;

fn main() {
    App::new() 
        .add_systems(Startup, setup)
        .add_systems(
            Update, 
            (
                handle_cursor_input,
                handle_mouse_hover,
                update_cursor_position,
            ).chain(),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct MenuCursor {
    selected_index: usize,
}

#[derive(Component)]
struct MenuItem; // Marker component for filtering

// Stores menu item entities in order (index = position in Vec)
#[derive(Resource)]
struct MenuItems(Vec<Entity>);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
    commands.spawn((
        Text2d::new("Sudoku"),
        Transform::from_translation(Vec3::new(0.0, TITLE_Y, 0.0)),
    ));
    
    // Spawn menu items and store their entity references in order
    let menu_entities: Vec<Entity> = MENU_ITEMS
        .iter()
        .enumerate()
        .map(|(index, &item_text)| {
            let y_pos = MENU_START_Y - (index as f32 * MENU_SPACING);
            commands.spawn((
                Text2d::new(item_text),
                Transform::from_translation(Vec3::new(0.0, y_pos, 0.0)),
                MenuItem,
            )).id()
        })
        .collect();
    
    commands.insert_resource(MenuItems(menu_entities));
    
    commands.spawn((
        Text2d::new(">"),
        Transform::from_translation(Vec3::new(CURSOR_X_OFFSET, MENU_START_Y, 0.0)),
        MenuCursor { selected_index: 0 },
    ));
}

fn handle_cursor_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cursor: Single<&mut MenuCursor>,
) {
    use KeyCode::*;
    let max_index = MENU_ITEMS.len() - 1;
    
    if keyboard.any_just_pressed([ArrowDown, KeyS]) {
        cursor.selected_index = (cursor.selected_index + 1).min(max_index);
    }
    
    if keyboard.any_just_pressed([ArrowUp, KeyW]) {
        cursor.selected_index = cursor.selected_index.saturating_sub(1);
    }
}

fn handle_mouse_hover(
    mut cursor_moved: MessageReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    menu_items: Res<MenuItems>,
    transforms: Query<&Transform, With<MenuItem>>,
    mut menu_cursor: Single<&mut MenuCursor>,
) {
    let Some(cursor_event) = cursor_moved.read().last() else {
        return; // No movement this frame
    };
    
    let world_pos = camera.0
        .viewport_to_world_2d(camera.1, cursor_event.position)
        .expect("viewport conversion should succeed");
    
    // Check each menu item by index
    for (index, &entity) in menu_items.0.iter().enumerate() {
        if let Ok(item_transform) = transforms.get(entity) {
            let item_pos = item_transform.translation.truncate();
            
            if world_pos.distance(item_pos) < HOVER_THRESHOLD {
                menu_cursor.selected_index = index;
                break;
            }
        }
    }
}

fn update_cursor_position(
    mut cursor_query: Query<(&MenuCursor, &mut Transform), Changed<MenuCursor>>,
    menu_items: Res<MenuItems>,
    transforms: Query<&Transform, Without<MenuCursor>>,
) {
    let Ok((menu_cursor, mut cursor_transform)) = cursor_query.single_mut() else {
        return; // No cursor changed this frame
    };
    
    // Direct entity lookup - O(1) instead of O(n) search
    if let Some(&entity) = menu_items.0.get(menu_cursor.selected_index) {
        if let Ok(item_transform) = transforms.get(entity) {
            cursor_transform.translation.y = item_transform.translation.y;
        }
    }
}