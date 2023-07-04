use super::{almost_zero, left_margin, GeneralInfo, CARD_WIDTH};
use crate::{
	ext::{EntityCommandsExt, SpawnToParent, TransformExt},
	textmesh::{get_text_mesh_with_bbox, BoundingBoxExt},
};
use bevy::prelude::*;

impl GeneralInfo {
	const names_text_size: f32 = 0.5;
	const names_text_y: f32 = 0.2;
	const names_margin: f32 = 0.1;

	const name_colour: Color = Color::WHITE;
	const aka_name_colour: Color = Color::GRAY;

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
		let mut parent = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation),
			..default()
		});
		parent.name("GeneralInfo row (parent)");

		let (name_mesh, name_bbox) =
			get_text_mesh_with_bbox(&self.name.to_uppercase(), GeneralInfo::names_text_size);
		let name_translation = Vec3::new(
			-CARD_WIDTH / 2. + left_margin + name_bbox.size().x / 2.,
			GeneralInfo::names_text_y,
			0.,
		);

		parent.with_children(|parent| {
			let name_transform = Transform::from_translation(Vec3::Z * almost_zero)
				.translate(name_translation)
				.translate(name_bbox.get_required_text_offset());
			let material = StandardMaterial {
				base_color: GeneralInfo::name_colour,
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
		});

		if let Some(aka_name) = &self.aka_name {
			parent.with_children(move |parent| {
				let (aka_mesh, aka_bbox) = get_text_mesh_with_bbox(aka_name, GeneralInfo::names_text_size);
				let aka_translation = Vec3::new(
					-CARD_WIDTH / 2. + left_margin + aka_bbox.size().x / 2. + name_bbox.size().x,
					GeneralInfo::names_text_y,
					0.,
				);
				let aka_transform = Transform::from_translation(Vec3::Z * almost_zero)
					.translate(aka_translation)
					.translate(Vec3::X * GeneralInfo::names_margin)
					.translate(aka_bbox.get_required_text_offset());
				let material = StandardMaterial {
					base_color: GeneralInfo::aka_name_colour,
					unlit: true,
					..default()
				};
				parent
					.spawn(PbrBundle {
						transform: aka_transform,
						mesh: meshs.add(aka_mesh),
						material: mat.add(material),
						..default()
					})
					.name("Aka name");
			});
		}

		parent.id()
	}
}
