use bevy::{
	prelude::*,
	window::PresentMode,
};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Hello World!"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(show_popup)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(Camera2dBundle::default());
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
    
	info!("Hello world!");
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
			info!("New Picture Popping Up!");
           
		}
	}
}
