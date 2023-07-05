use std::num::NonZeroU8;

use super::{almost_zero, left_margin, CARD_WIDTH};
use crate::{
	ext::{EntityCommandsExt, IntoAssetPath, SpawnToParent, TransformExt},
	textmesh::{get_text_mesh_with_bbox, BoundingBoxExt},
	texture_2d,
};
use bevy::prelude::*;
use strum::{Display, EnumIs, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControllerGeneralInfo {
	pub name: String,
	pub aka_name: Option<String>,
	pub gender: Gender,
	pub race: ClassRace,

	pub health: NonZeroU8,
}

/// Gender, including `Gender::Neither`
#[derive(Display, Debug, Clone, PartialEq, Eq, EnumIs, IntoStaticStr)]
pub enum Gender {
	Male,
	Female,
	Niether,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassRace {
	pub class: Class,
	pub race: Race,
}

// todo!(Implement race/class rendering, taking into account Controller + Orbital strike team classes);
/// Includes `ClassRace::Controller`
#[derive(Debug, Clone, PartialEq, Eq, EnumIs)]
pub enum Race {
	Human,
	PostHuman,
	Drone,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, EnumIs)]
pub enum Class {
	#[default]
	None,

	Unit707,
	OrbitalStrikeTeam,
}

impl ControllerGeneralInfo {
	const names_text_size: f32 = 0.5;
	const names_margin: f32 = 0.1;

	const name_colour: Color = Color::WHITE;
	const aka_name_colour: Color = Color::GRAY;

	const upper_row_y: f32 = 0.2;
	const lower_row_y: f32 = -0.3;

	/// Height of general info row (total)
	pub const height: f32 = 0.98;
}

impl SpawnToParent for ControllerGeneralInfo {
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

		let (name_mesh, name_bbox) = get_text_mesh_with_bbox(
			&self.name.to_uppercase(),
			ControllerGeneralInfo::names_text_size,
		);
		let name_translation = Vec3::new(
			-CARD_WIDTH / 2. + left_margin + name_bbox.size().x / 2.,
			ControllerGeneralInfo::upper_row_y,
			0.,
		);

		// name
		parent.with_children(|parent| {
			let name_transform = Transform::from_translation(Vec3::Z * almost_zero)
				.translate(name_translation)
				.translate(name_bbox.get_required_text_offset());
			let material = StandardMaterial {
				base_color: ControllerGeneralInfo::name_colour,
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

		// aka name
		if let Some(aka_name) = &self.aka_name {
			parent.with_children(|parent| {
				let (aka_mesh, aka_bbox) =
					get_text_mesh_with_bbox(aka_name, ControllerGeneralInfo::names_text_size);
				let aka_translation = Vec3::new(
					-CARD_WIDTH / 2. + left_margin + aka_bbox.size().x / 2. + name_bbox.size().x,
					ControllerGeneralInfo::upper_row_y,
					0.,
				);
				let aka_transform = Transform::from_translation(Vec3::Z * almost_zero)
					.translate(aka_translation)
					.translate(Vec3::X * ControllerGeneralInfo::names_margin)
					.translate(aka_bbox.get_required_text_offset());
				let material = StandardMaterial {
					base_color: ControllerGeneralInfo::aka_name_colour,
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

		// gender
		parent.with_children(|parent| {
			let gender_transform = Vec3::new(-CARD_WIDTH / 2. + left_margin + Gender::width / 2., ControllerGeneralInfo::lower_row_y, almost_zero);
			self.gender.spawn_using_entity_commands(parent, gender_transform, (meshs, mat, ass));
		});

		parent.id()
	}
}

impl IntoAssetPath for Gender {
	fn get_asset_path(&self) -> String {
		match self {
			Gender::Male => "card-icons/gender-male.png",
			Gender::Female => "card-icons/gender-female.png",
			Gender::Niether => "card-icons/gender-niether.png",
		}
		.to_string()
	}
}

impl Gender {
	pub const width: f32 = 2.5 / 10.;
	const height: f32 = 2.1 / 10.;
}

impl SpawnToParent for Gender {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::ext::mutASS,
	) -> Entity {
		let mut icon = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation),
			material: mat.add(texture_2d(ass.load(self.get_asset_path()))),
			mesh: meshs.add(shape::Quad::new(Vec2::new(Gender::width, Gender::height)).into()),
			..default()
		});
		icon.name(format!("Gender {}", self));
		icon.id()
	}
}
