use avian2d::prelude::{Collider, PhysicsDebugPlugin, PhysicsPickingPlugin, RigidBody};
use avian2d::PhysicsPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetServer;
use bevy::prelude::{AppExtStates, Camera2d, Commands, Component, ContainsEntity, Entity, FixedUpdate, Handle, Image, ImagePlugin, IntoScheduleConfigs, Mut, NextState, Pointer, Pressed, Query, Res, ResMut, Resource, Sprite, State, States, Transform, Trigger, Vec3, With};
use bevy::utils::default;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(100.0),
            PhysicsDebugPlugin::default(),
            PhysicsPickingPlugin,
        ))
        .add_systems(Startup, (load_resources, startup.after(load_resources)))
        .add_systems(FixedUpdate, system_additional_wins_check_system)
        .init_state::<GameState>()
        .run();
}

#[derive(Resource, Clone)]
struct GameAssets {
    empty_texture: Handle<Image>,
    x_texture: Handle<Image>,
    zero_texture: Handle<Image>,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    XTurn,
    ZeroTurn,
    XWins,
    ZeroWins,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayableCellState {
    X,
    Zero,
    Empty,
}

#[derive(Component)]
pub struct PlayableCell {
    state: PlayableCellState,
    x_index: i32,
    y_index: i32,
}

#[derive(Component)]
pub struct GameField {
    field: [[PlayableCellState; 3]; 3],
}

impl GameField {
    fn default() -> Self {
        Self {
            field: [
                [PlayableCellState::Empty, PlayableCellState::Empty, PlayableCellState::Empty],
                [PlayableCellState::Empty, PlayableCellState::Empty, PlayableCellState::Empty],
                [PlayableCellState::Empty, PlayableCellState::Empty, PlayableCellState::Empty],
            ]
        }
    }

    fn put_on_field(
        &mut self,
        x: usize,
        y: usize,
        state: PlayableCellState
    ) {
        let current_state = self.field[x][y];
        if current_state == PlayableCellState::Empty {
            self.field[x][y] = state;
        }
    }
}

fn load_resources(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let assets = GameAssets {
        x_texture: asset_server.load(
            "ui-pack/PNG/Yellow/Default/crosshair_color_c.png"
        ),
        zero_texture: asset_server.load(
            "ui-pack/PNG/Yellow/Default/crosshair_color_a.png"
        ),
        empty_texture: asset_server.load(
            "ui-pack/PNG/Yellow/Default/bar_square_large_square.png"
        ),
    };

    commands.insert_resource(assets);
}

fn startup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn(Camera2d);
    commands.spawn(GameField::default());

    let initial_cell_width: f32 = 24.0;
    let initial_cell_height: f32 = 24.0;
    let cell_scale_factor: f32 = 5.0;

    let cell_width: f32 = initial_cell_width * cell_scale_factor;
    let cell_height: f32 = initial_cell_height * cell_scale_factor;

    let empty_cell_sprite = Sprite::from_image(game_assets.empty_texture.clone());
    let scale = Vec3::new(cell_scale_factor, cell_scale_factor, cell_scale_factor);

    for i in 0..3 {
        for j in 0..3 {
            let current_position = Vec3::new(
                i as f32 * cell_width,
                j as f32 * cell_height,
                0.0
            );
            commands.spawn((
                PlayableCell {
                    state: PlayableCellState::Empty,
                    x_index: i,
                    y_index: j,
                },
                empty_cell_sprite.clone(),
                Collider::rectangle(initial_cell_width, initial_cell_height),
                RigidBody::Static,
                Transform {
                    translation: current_position,
                    scale: scale,
                    ..default()
                },
            )).observe(|trigger: Trigger<Pointer<Pressed>>,
                        mut commands: Commands,
                        state: Res<State<GameState>>,
                        game_assets: Res<GameAssets>,
                        mut next_state: ResMut<NextState<GameState>>,
                        mut cell_q: Query<(&mut PlayableCell)>,
                        mut game_field_q: Query<&mut GameField>,| {
                let target_entity = trigger.target.entity();
                handle_cell_pressed_system(
                    target_entity,
                    cell_q,
                    state,
                    next_state,
                    game_field_q,
                    commands,
                    game_assets,
                );
            });
        }
    }
}

fn handle_cell_pressed_system(
    target_entity: Entity,
    mut cell_q: Query<(&mut PlayableCell)>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_field_q: Query<&mut GameField>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    if let Ok((mut playable_cell)) = cell_q.get_mut(target_entity) {
        if let Ok(mut game_field) = game_field_q.single_mut() {
            match *state.get() {
                GameState::XTurn => {
                    playable_cell.state = PlayableCellState::X;
                    game_field.put_on_field(
                        playable_cell.x_index as usize,
                        playable_cell.y_index as usize,
                        PlayableCellState::X
                    );
                    commands.entity(target_entity).insert(Sprite::from_image(game_assets.x_texture.clone()));
                    next_state.set(resolve_next_game_state(GameState::XTurn, game_field));
                }
                GameState::ZeroTurn => {
                    playable_cell.state = PlayableCellState::Zero;
                    game_field.put_on_field(
                        playable_cell.x_index as usize,
                        playable_cell.y_index as usize,
                        PlayableCellState::Zero
                    );
                    commands.entity(target_entity).insert(Sprite::from_image(game_assets.zero_texture.clone()));
                    next_state.set(resolve_next_game_state(GameState::ZeroTurn, game_field));
                }
                GameState::XWins => {
                }
                GameState::ZeroWins => {
                }
            }
        }
    }
}

fn system_additional_wins_check_system(
    state: Res<State<GameState>>,
    mut game_field_q: Query<Entity, With<GameField>>,
    mut commands: Commands,
) {
    if let Ok(entity) = game_field_q.single_mut() {
        match *state.get() {
            GameState::XTurn => {
            }
            GameState::ZeroTurn => {
            }
            GameState::XWins => {
                println!("X wins!");
                commands.entity(entity).despawn();
            }
            GameState::ZeroWins => {
                println!("0 wins!");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn resolve_next_game_state(
    current_state: GameState,
    game_field: Mut<GameField>,
) -> GameState {
    match check_winner(game_field) {
        None => {
            if current_state == GameState::XTurn {
                GameState::ZeroTurn
            } else {
                GameState::XTurn
            }
        }
        Some(it) => {
            match it {
                PlayableCellState::X => {
                    GameState::XWins
                }
                PlayableCellState::Zero => {
                    GameState::ZeroWins
                }
                PlayableCellState::Empty => {
                    panic!("Wrong state!")
                }
            }
        }
    }
}

const WIN_PATTERNS: [[(usize, usize); 3]; 8] = [
    // Горизонтали
    [(0,0), (0,1), (0,2)],
    [(1,0), (1,1), (1,2)],
    [(2,0), (2,1), (2,2)],
    // Вертикали
    [(0,0), (1,0), (2,0)],
    [(0,1), (1,1), (2,1)],
    [(0,2), (1,2), (2,2)],
    // Диагонали
    [(0,0), (1,1), (2,2)],
    [(0,2), (1,1), (2,0)],
];

fn check_winner(game_field: Mut<GameField>) -> Option<PlayableCellState> {
    for &[a, b, c] in &WIN_PATTERNS {
        let (ax, ay) = a;
        let (bx, by) = b;
        let (cx, cy) = c;

        if game_field.field[ax][ay] != PlayableCellState::Empty
            && game_field.field[ax][ay] == game_field.field[bx][by]
            && game_field.field[bx][by] == game_field.field[cx][cy] {
            return Some(game_field.field[ax][ay]);
        }
    }

    None
}


