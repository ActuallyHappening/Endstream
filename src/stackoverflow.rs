pub fn render_text(
	at: Transform,
	content: &str,
	font_file: &str,
	commands: &mut Commands,
	asset_server: ResMut<AssetServer>,
	materials: ResMut<Assets<Mesh>>,
	meshs: ResMut<Assets<Mesh>>,
) {
	// load font
	let font = asset_server.load(font_file);

	// create text
	let text_something_idk = todo!();

	commands.spawn(text_something_idk);
}

fn text_to_image_runtime(text: &str, asset_server: Res<AssetServer>) -> Handle<Image> {
	use text_to_png::TextRenderer;

	let renderer = TextRenderer::try_new_with_ttf_font_data(include_bytes!("font.ttf"))
		.expect("Could not load file");

	let text_png = renderer
		.render_text_to_png_data(text, 42, "white")
		.expect("Couldn't custom render text");

	// NOTE: This line will break on WASM
	std::fs::write(format!("assets/text-file-{}.png", text), text_png.data).expect("File works + not on WASM");

	asset_server.load(format!("text-file-{}.png", text))
}

pub fn render_text(
	at: Transform,
	content: &str,
	font_file: &str,
	commands: &mut Commands,
	asset_server: ResMut<AssetServer>,
	materials: ResMut<Assets<Mesh>>,
	meshs: ResMut<Assets<Mesh>>,
) {
	// load font
	let font = asset_server.load(font_file);

	// create text
	let text_something_idk = todo!();

	commands.spawn(text_something_idk);
}