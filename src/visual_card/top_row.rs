use bevy::prelude::*;

use crate::{agendas::AgendaCost, utils::{SpawnToParent, EntityCommandsExt}, visual_card::{CARD_WIDTH, right_margin, left_margin}};

use super::{century::Century, gear_slots::GearSlots};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopRow {
	pub start_century: Option<Century>,
	pub activation_cost: Option<AgendaCost>,
	pub gear_slots: GearSlots,
}

impl SpawnToParent for TopRow {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::utils::mutASS,
	) -> Entity {
		let mut top_row = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation),
			..default()
		});
		top_row.name("Top row");

		top_row.with_children(|parent| {
			// spawn century icon
			if let Some(start_century) = &self.start_century {
				start_century.spawn_using_entity_commands(parent, Vec3::ZERO, (meshs, mat, ass));
			}
			// end spawn century

			// spawn agenda cost
			if let Some(agenda_cost) = &self.activation_cost {
				let x = -(CARD_WIDTH / 2.0) + agenda_cost.width() / 2.0 + left_margin;
				agenda_cost.spawn_using_entity_commands(parent, Vec3::X * x, (meshs, mat, ass));
			}
			// end agenda cost

			/* #region gear slots */
			const gear_slots_x: f32 = CARD_WIDTH / 2. - right_margin - GearSlots::width / 2.;
			self.gear_slots.spawn_using_entity_commands(
				parent,
				Vec3::X * gear_slots_x,
				(meshs, mat, ass),
			);
			/* #endregion */
		});

		top_row.id()
	}
}
