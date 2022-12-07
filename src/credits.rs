use bevy::prelude::*;
use std::time::Duration;

use crate::{
	GameState,
};

pub struct CreditsPlugin;

#[derive(Component)]
struct Credit;

#[derive(Component, Deref, DerefMut)]
struct CreditsTimer{
	timer: Timer,
}

#[derive(Component)]
struct OnScreen;

#[derive(Component)]
struct NotDone;

impl Plugin for CreditsPlugin{
    fn build(&self, app: &mut App){
        app
			.add_startup_system(setup_credits)
			.add_system_set(SystemSet::on_update(GameState::Credits)
				.with_system(show_popup)
				.with_system(check_if_done)
			)
			.add_system_set(SystemSet::on_enter(GameState::Credits)
				.with_system(insert_timers)
			)
			.add_system_set(SystemSet::on_exit(GameState::Credits)
				//.with_system()
			);
    }
}

fn setup_credits(mut commands: Commands, asset_server: Res<AssetServer>,){
	commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Mark_Marquez.png"),
		visibility: Visibility {
			is_visible: false
		},
        ..default()
    })
	.insert(Credit);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Yuanheng_Qu.png"),
		visibility: Visibility {
			is_visible: false
		},
        ..default()
    })
	.insert(Credit);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("John_Stroud.png"),
		visibility: Visibility {
			is_visible: false
		},
        ..default()
    })
	.insert(Credit);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Brian_Lucas.png"),
		visibility: Visibility {
			is_visible: false
		},
        ..default()
    })
	.insert(Credit);

	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("Yanding_Liu.png"),
		visibility: Visibility {
			is_visible: false
		},
		..default()
	})
	.insert(Credit);

	commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Michael_Schafer.png"),
		visibility: Visibility {
			is_visible: false
		},
        ..default()
    })
	.insert(Credit);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Qirui_Liu.png"),
		visibility: Visibility {
			is_visible: false
		},
        ..default()
    })
	.insert(Credit);
}

fn insert_timers(
	mut credits: Query<Entity, With<Credit>>,
	mut commands: Commands,
){
	let mut secs: u64 = 1;
	for c in credits.iter_mut(){
		commands.entity(c)
		.insert(CreditsTimer {
			timer: Timer::new(Duration::from_secs(secs),false)})
		.insert(NotDone);
		secs = secs + 3;
	}
}

fn check_if_done(
	mut not: Query<Entity, (With<Credit>, With<NotDone>)>,
	mut game_state: ResMut<State<GameState>>,
){

	match not.iter_mut().next() {
		Some(_a) => {
		},
		None => {
			game_state.set(GameState::End).unwrap();
		},
	}

}

fn show_popup(
	time: Res<Time>,
	mut creditlist: Query<(&mut Visibility, &mut CreditsTimer, Entity), (With<Credit>, Without<OnScreen>)>,
	mut commands: Commands,
	mut on_screen: Query<(&mut Visibility, &mut CreditsTimer, Entity), (With<Credit>, With<OnScreen>)>,
) {
	for (mut v, mut timer, s) in on_screen.iter_mut(){
		timer.tick(time.delta());
		if timer.just_finished(){
			v.is_visible = false;
			commands.entity(s).remove::<OnScreen>().remove::<CreditsTimer>().remove::<NotDone>();
			continue;
		}
	}
	for (mut v, mut timer, e) in creditlist.iter_mut(){
		timer.tick(time.delta());
		if timer.just_finished(){
			commands.entity(e).insert(OnScreen);
			commands.entity(e).insert(CreditsTimer {
				timer: Timer::new(Duration::from_secs(3),false)});
			v.is_visible = true;
		}
	}
		//commands.entity(timer).despawn();

}
