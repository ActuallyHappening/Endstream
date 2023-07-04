use super::GeneralInfo;
use crate::{ext::SpawnToParent, textmesh::get_text_mesh_with_bbox};
use bevy::prelude::*;

impl GeneralInfo {
	const general_text_size: f32 = 0.3;
	pub const height: f32 = 0.98;
}

impl SpawnToParent for GeneralInfo {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut bevy::prelude::ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		ass: crate::ext::mutASS,
	) -> Entity {
		let name_mesh = get_text_mesh_with_bbox(&self.name, GeneralInfo::general_text_size);

		parent.spawn_empty().id()
	}
}
