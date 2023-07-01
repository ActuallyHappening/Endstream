pub enum Card {
	Controller(ControllerCard)
}

pub enum ControllerCard {
	HQ,
}

impl ControllerCard {
	pub fn get_asset_path(&self) -> &'static str {
		match self {
			ControllerCard::HQ => "assets/controllers/1-V4 - R - HQ 1 Ural 18 FINAL.jpg",
		}
	}
}


pub fn spawn_card() {

}