#![allow(non_upper_case_globals)]

use bevy_fly_camera::FlyCamera;
// use bevy_rapier3d::prelude::*;
use card::spawn_all_cards_debug;
use bevy::prelude::*;

use crate::ext::EntityCommandsExt;

mod card;
mod agendas;
mod textmesh;
mod ext;



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

fn texture_2d(texture_handle: Handle<Image>) -> StandardMaterial {
	StandardMaterial {
		base_color_texture: Some(texture_handle),
		unlit: true,
		alpha_mode: AlphaMode::Blend,
		cull_mode: None,
		depth_bias: level_0_depth_bias,
		..default()
	}
}
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
		.insert(FlyCamera::default())
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
			// PickableBundle::default(),    // Makes the entity pickable
			// RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
			// OnPointer::<Click>::run_callback(handle_plane_clicked),
		))
		// .insert(Collider::cuboid(100.0, DIF, 100.0))
		.name("Ground");
}
