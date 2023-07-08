#![allow(non_upper_case_globals)]
// #![feature(const_option)]
// #![feature(const_trait_impl)]

use card::spawn_all_cards_debug;
use bevy::prelude::*;

use crate::utils::EntityCommandsExt;

mod card;
mod agendas;
mod textmesh;
mod utils;
mod data;


pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(setup)
			// .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			// .add_plugin(RapierDebugRenderPlugin::default())
			.add_startup_system(spawn_all_cards_debug);
	}
}

/// Default depth bias (used in utils/texture_2d)
const level_0_depth_bias: f32 = 0.;
/// Used in card/agenda_cost.rs to avoid Z-fighting
const level_1_depth_bias: f32 = 1.0;

const CAMERA_POS: Vec3 = Vec3::new(5., 20., 1.);
const CAMERA_LOOKING_AT: Vec3 = Vec3::new(5., 5., 0.);

#[derive(Component)]
struct MainCamera;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// cam
	commands
		.spawn(Camera3dBundle {
			transform: Transform::from_translation(CAMERA_POS).looking_at(CAMERA_LOOKING_AT, Vec3::Y),
			..default()
		})
		.insert(MainCamera)
		.as_pick_camera()
		.name("Main camera");

	// light
	commands
		.spawn(PointLightBundle {
			point_light: PointLight {
				intensity: 50000.0,
				range: 2500.,
				shadows_enabled: true,
				..default()
			},
			transform: Transform::from_translation(CAMERA_POS + Vec3::Y * 10.),
			..default()
		})
		.name("Main light");

	// ground plane
	const DIF: f32 = 0.1;
	commands
		.spawn((
			PbrBundle {
				mesh: meshes.add(shape::Plane::from_size(500.0).into()),
				material: materials.add(Color::SILVER.into()),
				// transform to be behind, xy plane
				transform: Transform::from_xyz(0., -DIF, 0.),
				..default()
			},
		))
		.name("Ground");
}
