use std::num::NonZeroU8;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::IntoAssetPath;

#[derive(IntoStaticStr, EnumIter, EnumString, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SingleAgendaType {
	Military,
	Science,
	Politics,
	Wild,
}

/// Represents agenda symbol/s for a single cost rectangle
#[derive(Debug, Clone)]
pub enum AgendaType {
	Single(SingleAgendaType),
	Double(SingleAgendaType, SingleAgendaType),
}

/// Represents the semi-rectangular agenda cost icons
#[derive(Debug, Clone)]
pub enum AgendaCostItem {
	Single {
		agenda_type: AgendaType,
		count: NonZeroU8,
	},
	Double {
		agenda_types: [AgendaType; 2],
		count: NonZeroU8,
	},
}

#[derive(Debug, Clone)]
pub enum AgendaCost {
	One {
		only: AgendaCostItem,
	},
	Two {
		first: AgendaCostItem,
		second: AgendaCostItem,
	},
}

impl IntoAssetPath for AgendaType {
	fn get_asset_path(&self,) -> String {
			
	}
}