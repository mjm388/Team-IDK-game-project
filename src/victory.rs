
use bevy::{prelude::*
};
use std::time::Duration;

use crate::{BossTrigger};
use crate::GameState;

pub struct VictoryPlugin;

#[derive(Component)]
pub struct Victory;

#[derive(Component, Deref, DerefMut)]
struct VictoryTimer{
	timer: Timer,
}

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_victory)
        .add_system_set(SystemSet::on_update(GameState::Victory)
			.with_system(check_timer)
		)
        .add_system_set(SystemSet::on_enter(GameState::Victory)
            .with_system(render)
            .with_system(insert_timer)
        )
        .add_system_set(SystemSet::on_exit(GameState::Victory)
            .with_system(derender)
        );
    }
}

fn derender(
    mut screen: Query<(&mut Visibility, Entity), With<Victory>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = false;
    }
}

fn render(
    mut screen: Query<(&mut Visibility, Entity), With<Victory>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = true;
    }
}

fn insert_timer(
	mut victories: Query<Entity, With<Victory>>,
	mut commands: Commands,
){
	let secs: u64 = 3; //Screen stays for 3 seconds
	for c in victories.iter_mut(){
		commands.entity(c)
		.insert(VictoryTimer {
			timer: Timer::new(Duration::from_secs(secs),false)});
	}
}

fn check_timer(
	time: Res<Time>,
	mut victory_list: Query<&mut VictoryTimer, With<Victory>>,
    mut game_state: ResMut<State<GameState>>,
    mut boss_flag: Query<&mut BossTrigger>,
) {
    let boss_fight = boss_flag.single_mut();
	for mut timer in victory_list.iter_mut(){
		timer.tick(time.delta());
		if timer.just_finished() && boss_fight.boss_trigger {
            game_state.set(GameState::Credits).unwrap();
		} else if timer.just_finished() && !boss_fight.boss_trigger {
            game_state.set(GameState::Overworld).unwrap();
        }
	}

}

fn setup_victory(mut commands: Commands, asset_server: Res<AssetServer>,){   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Victory_Scene.png"),
        transform: Transform {
			translation: Vec3::new(0., 0., 900.),
			..default()
		},
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
    .insert(Victory);
}   