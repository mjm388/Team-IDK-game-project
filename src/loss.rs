
use bevy::{prelude::*
};
use std::time::Duration;

use crate::GameState;

pub struct LossPlugin;

#[derive(Component)]
pub struct Loss;

#[derive(Component, Deref, DerefMut)]
struct LossTimer{
	timer: Timer,
}

impl Plugin for LossPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_loss)
        .add_system_set(SystemSet::on_update(GameState::Loss)
				.with_system(check_timer)
		)
        .add_system_set(SystemSet::on_enter(GameState::Loss)
            .with_system(render)
            .with_system(insert_timer)
        )
        .add_system_set(SystemSet::on_exit(GameState::Loss)
            .with_system(derender)
        );
    }
}

fn derender(
    mut screen: Query<(&mut Visibility, Entity), With<Loss>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = false;
    }
}

fn render(
    mut screen: Query<(&mut Visibility, Entity), With<Loss>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = true;
    }
}

fn insert_timer(
	mut losses: Query<Entity, With<Loss>>,
	mut commands: Commands,
){
	let secs: u64 = 3; //Screen stays for 3 seconds
	for c in losses.iter_mut(){
		commands.entity(c)
		.insert(LossTimer {
			timer: Timer::new(Duration::from_secs(secs),false)});
	}
}

fn check_timer(
	time: Res<Time>,
	mut loss_list: Query<&mut LossTimer, With<Loss>>,
    mut game_state: ResMut<State<GameState>>,
) {
	for mut timer in loss_list.iter_mut(){
		timer.tick(time.delta());
		if timer.just_finished() {
            game_state.set(GameState::Credits).unwrap();
		}
	}

}

fn setup_loss(mut commands: Commands, asset_server: Res<AssetServer>,){   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Death_Scene.png"),
        transform: Transform {
			translation: Vec3::new(0., 0., 900.),
			..default()
		},
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
    .insert(Loss);
}   