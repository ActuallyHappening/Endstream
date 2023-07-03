use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_fly_camera::FlyCamera;
use bevy_mod_picking::{backends::rapier::RapierPickTarget, prelude::*};
use bevy_rapier3d::prelude::*;
use card::spawn_all_cards_debug;

mod card;
mod agendas;

pub type ASS<'a> = (
	ResMut<'a, Assets<Mesh>>,
	ResMut<'a, Assets<StandardMaterial>>,
	ResMut<'a, AssetServer>,
);

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

trait EntityCommandsExt {
	fn with_picking(&mut self) -> &mut Self;

	fn as_pick_camera(&mut self) -> &mut Self;

	fn name(&mut self, name: &'static str) -> &mut Self;
}

impl EntityCommandsExt for EntityCommands<'_, '_, '_> {
	/// Adds bevy_mod_picking magic
	fn with_picking(&mut self) -> &mut Self {
		self
			.insert(PickableBundle::default())
			.insert(RapierPickTarget)
			.insert(RaycastPickTarget::default())
	}

	fn as_pick_camera(&mut self) -> &mut Self {
		self
			.insert(RapierPickCamera::default())
			.insert(RaycastPickCamera::default())
	}

	fn name(&mut self, name: &'static str) -> &mut Self {
		self.insert(Name::new(name))
	}
}

trait IntoAssetPath {
	fn get_asset_path(&self,) -> String;
}

const CAMERA_POS: Vec3 = Vec3::new(5., 30., 1.);
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
		.insert(Collider::cuboid(100.0, DIF, 100.0))
		.name("Ground");
}
