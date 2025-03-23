use bevy::prelude::*;
use bevy_wasm::WasmPlugin;

// Constants for grid size
const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 50.0;

// Components
#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    count: u8, // Number of bubbles in the cell
    player: Option<u8>, // Which player owns the cell
}

// Plugin for game logic
struct ChainReactionPlugin;

impl Plugin for ChainReactionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup) // Updated for Bevy 0.12
            .add_systems(Update, handle_input) // Updated for Bevy 0.12
            .add_systems(Update, update_cells); // Updated for Bevy 0.12
    }
}

// Setup game board
fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set window size
    let mut window = windows.single_mut();
    window.resolution.set(600.0, 600.0);

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn grid cells
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let cell_pos = Vec3::new(
                x as f32 * CELL_SIZE - 250.0,
                y as f32 * CELL_SIZE - 250.0,
                0.0,
            );

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(CELL_SIZE - 5.0, CELL_SIZE - 5.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(cell_pos),
                    ..default()
                })
                .insert(Cell {
                    x,
                    y,
                    count: 0,
                    player: None,
                });
        }
    }
}

// Handle user input
fn handle_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window>,
    mut query: Query<(Entity, &mut Cell, &mut Sprite)>,
) {
    let window = q_windows.single();
    
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let grid_x = (position.x / CELL_SIZE) as usize;
            let grid_y = (position.y / CELL_SIZE) as usize;

            for (entity, mut cell, mut sprite) in query.iter_mut() {
                if cell.x == grid_x && cell.y == grid_y {
                    cell.count += 1;
                    cell.player = Some(1); // Player 1
                    
                    // Change color based on count
                    sprite.color = match cell.count {
                        1 => Color::YELLOW,
                        2 => Color::ORANGE,
                        3 => Color::RED,
                        _ => Color::WHITE,
                    };

                    // TODO: Implement chain reaction
                    break;
                }
            }
        }
    }
}

// Update cells (TODO: Implement chain reaction)
fn update_cells(
    mut query: Query<(Entity, &mut Cell, &mut Sprite)>,
) {
    for (_entity, mut cell, _sprite) in query.iter_mut() {
        if cell.count > 3 {
            cell.count = 0;
            cell.player = None;

            // TODO: Spread bubbles to adjacent cells
        }
    }
}

// Bevy App Runner
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WasmPlugin) 
        .add_plugins(ChainReactionPlugin) // Use .add_plugins() instead of .add_plugin()
        .run();
}
