use bevy::prelude::*;

use crate::{
	GameState,
};

pub struct CreditsPlugin;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

impl Plugin for CreditsPlugin{
    fn build(&self, app: &mut App){
        app
			.add_system_set(SystemSet::on_update(GameState::Credits)
				.with_system(show_popup)
			)
			.add_system_set(SystemSet::on_enter(GameState::Credits)
				.with_system(play_credits)
			)
			.add_system_set(SystemSet::on_exit(GameState::Credits));
    }
}

fn play_credits(mut commands: Commands, asset_server: Res<AssetServer>,){
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Mark_Marquez.png"),
			..default()
		});
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Yuanheng_Qu.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(5., false)));
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("John_Stroud.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(10., false)));
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Brian_Lucas.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(15., false)));
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Yanding_Liu.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(20., false)));
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Michael_Schafer.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(25., false)));
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Qirui_Liu.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(30., false)));
}

fn show_popup(
	time: Res<Time>,
	mut popup: Query<(&mut PopupTimer, &mut Transform)>
) {
    let mut count = 1.;
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
        count = count+1.;
		if timer.just_finished() {
			transform.translation.z = count;
		}
	}
}
