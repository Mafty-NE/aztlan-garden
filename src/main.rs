use bevy::prelude::*;
use rand::random;

const TILE_SIZE: f32 = 32.0;
const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct TilePosition {
    x: u32,
    y: u32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum TileType {
    Grass,
    Dirt,
    Water,
    Crop,
}

impl TileType {
    fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::GREEN,
            TileType::Dirt => Color::rgb(0.5, 0.25, 0.1),
            TileType::Water => Color::BLUE,
            TileType::Crop => Color::rgb(0.1, 0.5, 0.1),
        }
    }
}

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
struct SelectedTileType(TileType);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SelectedTileType(TileType::Grass))
        .add_systems(Startup, (setup_camera, spawn_tiles, setup_ui))
        .add_systems(Update, (mouse_click_system, tile_hover_system, tile_type_button_system))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_tiles(mut commands: Commands) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let pos_x = x as f32 * TILE_SIZE - (GRID_WIDTH as f32 * TILE_SIZE / 2.0);
            let pos_y = y as f32 * TILE_SIZE - (GRID_HEIGHT as f32 * TILE_SIZE / 2.0);

            let tile_type = match random::<u8>() % 4 {
                0 => TileType::Grass,
                1 => TileType::Dirt,
                2 => TileType::Water,
                _ => TileType::Crop,
            };

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: tile_type.color(),
                        custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..default()
                })
                .insert(Tile)
                .insert(TilePosition { x, y })
                .insert(tile_type);
        }
    }
}

fn mouse_click_system(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut tiles: Query<(&mut Sprite, &Transform, &mut TileType)>,
    selected: Res<SelectedTileType>,
) {
    let window = windows.single();
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single();
            if let Some(world_pos) = camera
                .viewport_to_world(camera_transform, cursor_pos)
                .map(|r| r.origin.truncate())
            {
                for (mut sprite, transform, mut tile_type) in &mut tiles {
                    let pos = transform.translation.truncate();
                    let half_size = TILE_SIZE / 2.0;
                    let in_x = (world_pos.x - pos.x).abs() < half_size;
                    let in_y = (world_pos.y - pos.y).abs() < half_size;

                    if in_x && in_y {
                        *tile_type = selected.0;
                        sprite.color = tile_type.color();
                    }
                }
            }
        }
    }
}

fn tile_hover_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut tiles: Query<(&Transform, &mut Sprite, &TileType)>,
) {
    let window = windows.single();
    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_transform) = camera_q.single();
        if let Some(world_pos) = camera
            .viewport_to_world(camera_transform, cursor_pos)
            .map(|r| r.origin.truncate())
        {
            for (transform, mut sprite, tile_type) in &mut tiles {
                let pos = transform.translation.truncate();
                let half_size = TILE_SIZE / 2.0;
                let in_x = (world_pos.x - pos.x).abs() < half_size;
                let in_y = (world_pos.y - pos.y).abs() < half_size;

                if in_x && in_y {
                    sprite.color = Color::YELLOW;
                } else {
                    sprite.color = tile_type.color();
                }
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        for tile_type in [TileType::Grass, TileType::Dirt, TileType::Water, TileType::Crop] {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(80.0),
                        height: Val::Px(40.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(tile_type.color()),
                    ..Default::default()
                },
                tile_type,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    format!("{:?}", tile_type),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ));
            });
        }
    });
}

fn tile_type_button_system(
    interaction_query: Query<(&Interaction, &TileType, &BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut selected: ResMut<SelectedTileType>,
) {
    for (interaction, tile_type, _color) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                selected.0 = *tile_type;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
