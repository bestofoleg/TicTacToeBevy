use crate::core::check_winner;
use crate::resources::GameAssets;
use crate::{GameField, GameState, PlayableCell, PlayableCellState};
use avian2d::prelude::{Collider, RigidBody};
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{default, Added, Camera2d, Commands, ContainsEntity, Entity, EventWriter, Mut, NextState, Pointer, Pressed, Query, Res, ResMut, Sprite, State, Transform, Trigger, With};
use bevy_egui::{egui, EguiContexts};

// Startup systems

pub fn load_resources(
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

pub fn startup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn(Camera2d);

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
                (i - 1) as f32 * cell_width,
                (j - 1) as f32 * cell_height,
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
                        commands: Commands,
                        state: Res<State<GameState>>,
                        game_assets: Res<GameAssets>,
                        next_state: ResMut<NextState<GameState>>,
                        cell_q: Query<&mut PlayableCell>,
                        game_field_q: Query<&mut GameField>,| {
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

// UI Systems

pub fn ui_example_system(mut contexts: EguiContexts,
                     state: Res<State<GameState>>,
                     mut next_state: ResMut<NextState<GameState>>,
                     game_assets: Res<GameAssets>,
                     mut commands: Commands,
                     mut game_field_q: Query<Entity, With<GameField>>,
                     playable_cell: Query<(Entity, &mut PlayableCell), With<PlayableCell>>,
                     mut app_exit_events: EventWriter<AppExit>) {
    if let Ok(ctx) = contexts.ctx_mut() {
        match *state.get() {
            GameState::Draw => {
                if let Ok(entity) = game_field_q.single_mut() {
                    commands.entity(entity).despawn();
                }
                egui::Window::new("Game summary")
                    .show(ctx, |ui| {
                        ui.add(egui::Label::new("Draw!"));
                        if ui.add(egui::Button::new("Done!")).clicked() {
                            next_state.set(GameState::BeforeGame);
                        }
                    });
            }
            GameState::BeforeGame => {
                egui::Window::new("Main menu")
                    .show(ctx, |ui| {
                        if ui.add(egui::Button::new("New Game")).clicked() {
                            reinit_playable_cells(
                                playable_cell,
                                commands,
                                game_assets,
                            );
                        }
                        if ui.add(egui::Button::new("Quit")).clicked() {
                            app_exit_events.write(AppExit::Success);
                        }
                    });
            }
            GameState::XTurn => {}
            GameState::ZeroTurn => {}
            GameState::XWins => {
                if let Ok(entity) = game_field_q.single_mut() {
                    commands.entity(entity).despawn();
                }
                egui::Window::new("Game summary")
                    .show(ctx, |ui| {
                        ui.add(egui::Label::new("X wins!"));
                        if ui.add(egui::Button::new("Done!")).clicked() {
                            next_state.set(GameState::BeforeGame);
                        }
                    });
            }
            GameState::ZeroWins => {
                if let Ok(entity) = game_field_q.single_mut() {
                    commands.entity(entity).despawn();
                }
                egui::Window::new("Game summary")
                    .show(ctx, |ui| {
                        ui.add(egui::Label::new("0 wins!"));
                        if ui.add(egui::Button::new("Done!")).clicked() {
                            next_state.set(GameState::BeforeGame);
                        }
                    });
            }
        }
    } else {
        eprintln!("Failed to get Egui context");
    }
}

fn reinit_playable_cells(
    playable_cell: Query<(Entity, &mut PlayableCell), With<PlayableCell>>,
    mut commands: Commands,
    assets: Res<GameAssets>
) {
    for (entity, cell) in playable_cell.iter() {
        commands.entity(entity)
            .insert(Sprite::from_image(assets.empty_texture.clone()))
            .insert(PlayableCell {
                x_index: cell.x_index,
                y_index: cell.y_index,
                state: PlayableCellState::Empty
            });
    }
    commands.spawn(GameField::default());
}

pub fn on_game_field_created_system(
    mut query: Query<&mut GameField, Added<GameField>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(_) = query.single_mut() {
        next_state.set(GameState::XTurn);
    }
}

fn handle_cell_pressed_system(
    target_entity: Entity,
    mut cell_q: Query<&mut PlayableCell>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_field_q: Query<&mut GameField>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    if let Ok(mut playable_cell) = cell_q.get_mut(target_entity) {
        if let Ok(mut game_field) = game_field_q.single_mut() {
            match *state.get() {
                GameState::Draw => {
                }
                GameState::BeforeGame => {}
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

fn resolve_next_game_state(
    current_state: GameState,
    game_field: Mut<GameField>,
) -> GameState {
    match check_winner(&game_field) {
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
                    GameState::Draw
                }
            }
        }
    }
}
