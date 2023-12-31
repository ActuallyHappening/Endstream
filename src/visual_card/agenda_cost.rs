use crate::{
	agendas::{AgendaCost, AgendaType},
	visual_card::almost_zero,
	utils::{EntityCommandsExt, SpawnToParent, TransformExt},
	level_1_depth_bias,
	textmesh::{get_text_mesh, Fonts},
	utils::texture_2d,
};
use bevy::prelude::*;

impl SpawnToParent for AgendaCost {
	fn spawn_to_child_builder(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::utils::mutASS,
	) -> Entity {
		
		let transform = Transform::from_translation(translation);
		parent
			.spawn(PbrBundle {
				transform,
				..default()
			})
			.name("Activation cost parent")
			.with_children(|activation_cost| {
				// cost frame
				let cost_frame_shape = shape::Quad::new(Vec2::new(self.width(), AgendaCost::height));
				let material = texture_2d(ass.load(self.get_frame_asset_path()));
				let mut cost_frame = activation_cost.spawn(PbrBundle {
					mesh: meshs.add(cost_frame_shape.into()),
					material: mat.add(material),
					..default()
				});
				cost_frame.name("Cost frame");
				// end cost frame

				// agenda icons & numbers
				cost_frame.with_children(|cost_frame| {
					let mesh = shape::Quad::new(Vec2::new(AgendaType::width, AgendaType::height));
					let mesh = meshs.add(mesh.into());
					const number_size: f32 = 0.2;

					match &self {
						AgendaCost::One { only } => {
							// icon
							let material = StandardMaterial {
								depth_bias: level_1_depth_bias,
								..texture_2d(ass.load(only.agenda.get_icon_asset_path()))
							};
							// -0.05 is a slight offset, magic number
							let transform = Transform::from_xyz(0.0 - 0.05, -0.05, almost_zero);

							cost_frame
								.spawn(PbrBundle {
									transform,
									material: mat.add(material),
									mesh,
									..default()
								})
								.name("Only agenda");

							// number
							let (mesh, offset) =
								get_text_mesh(only.number.to_string(), number_size, Fonts::Heavy);
							let top_right_transform = Transform::from_xyz(
								cost_frame_shape.size.x / 2. - number_size / 2.,
								cost_frame_shape.size.y / 2. - number_size / 2.,
								almost_zero,
							);
							cost_frame
								.spawn(PbrBundle {
									transform: top_right_transform.translate(offset),
									mesh: meshs.add(mesh),
									material: mat.add(Color::WHITE.into()),
									..default()
								})
								.name("Number for only agenda");
						}
						AgendaCost::Two { first, second } => {
							// first
							let material = StandardMaterial {
								depth_bias: level_1_depth_bias,
								..texture_2d(ass.load(first.agenda.get_icon_asset_path()))
							};
							// -0.05 is a slight offset, magic number
							let transform = Transform::from_xyz(-self.width() / 4. - 0.05, -0.05, almost_zero);

							cost_frame
								.spawn(PbrBundle {
									transform,
									material: mat.add(material),
									mesh: mesh.clone(),
									..default()
								})
								.name("First agenda");

							// first number
							let (t_mesh, offset) =
								get_text_mesh(second.number.to_string(), number_size, Fonts::Heavy);
							let top_right_transform = Transform::from_xyz(
								-number_size / 2.,
								cost_frame_shape.size.y / 2. - number_size / 2.,
								almost_zero,
							);
							cost_frame
								.spawn(PbrBundle {
									transform: top_right_transform.translate(offset),
									mesh: meshs.add(t_mesh),
									material: mat.add(Color::WHITE.into()),
									..default()
								})
								.name("Number for only agenda");

							// second
							let material = StandardMaterial {
								depth_bias: level_1_depth_bias,
								..texture_2d(ass.load(second.agenda.get_icon_asset_path()))
							};
							// -0.05 is a slight offset, magic number
							let transform =
								Transform::from_xyz(self.width() / 4. - 0.05, 0.0 - 0.05, almost_zero);

							cost_frame
								.spawn(PbrBundle {
									transform,
									material: mat.add(material),
									mesh,
									..default()
								})
								.name("Second agenda");

							// second number
							let (mesh, offset) =
								get_text_mesh(second.number.to_string(), number_size, Fonts::Heavy);
							let top_right_transform = Transform::from_xyz(
								cost_frame_shape.size.x / 2. - number_size / 2.,
								cost_frame_shape.size.y / 2. - number_size / 2.,
								almost_zero,
							);
							cost_frame
								.spawn(PbrBundle {
									transform: top_right_transform.translate(offset),
									mesh: meshs.add(mesh),
									material: mat.add(Color::WHITE.into()),
									..default()
								})
								.name("Number for only agenda");
						}
					}
				});
				// end icons & numbers
			})
			.id()
	}
}
