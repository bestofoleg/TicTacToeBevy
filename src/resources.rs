use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct GameAssets {
    pub empty_texture: Handle<Image>,
    pub x_texture: Handle<Image>,
    pub zero_texture: Handle<Image>,
}