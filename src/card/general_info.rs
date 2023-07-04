use super::{almost_zero, left_margin, GeneralInfo, CARD_WIDTH};
use crate::{
	ext::{EntityCommandsExt, SpawnToParent, TransformExt},
	textmesh::{get_text_mesh_with_bbox, BoundingBoxExt},
};
use bevy::prelude::*;

impl GeneralInfo {
	const text_size: f32 = 0.4;
	const text_y: f32 = 0.2;

	/// Height of general info row (total)
	pub const height: f32 = 0.98;
}

impl SpawnToParent for GeneralInfo {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut bevy::prelude::ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::ext::mutASS,
	) -> Entity {
		let (name_mesh, name_bbox) =
			get_text_mesh_with_bbox(&self.name.to_uppercase(), GeneralInfo::text_size);
		let name_translation = Vec3::new(
			-CARD_WIDTH / 2. + left_margin + name_bbox.size().x / 2.,
			GeneralInfo::text_y,
			0.,
		);
		let name_transform = Transform::from_translation(Vec3::Z * almost_zero)
			.translate(translation)
			.translate(name_translation)
			.translate(name_bbox.get_required_text_offset());
		let material = StandardMaterial {
			base_color: Color::WHITE,
			unlit: true,
			..default()
		};
		parent
			.spawn(PbrBundle {
				transform: name_transform,
				mesh: meshs.add(name_mesh),
				material: mat.add(material),
				..default()
			})
			.name("Name");

		parent.spawn_empty().id()
	}
}
