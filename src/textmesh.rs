use bevy::prelude::*;
use meshtext::{MeshGenerator, MeshText, TextSection};

/// Generates text mesh
pub fn get_text_mesh_with_bbox(text: &str, pixel_size: f32) -> (Mesh, meshtext::BoundingBox) {
	let font_data = include_bytes!(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/assets/fonts/Oswald-Regular.ttf"
	));
	let font_data = include_bytes!("../assets/fonts/Oswald-Regular.ttf");

	let mut generator = MeshGenerator::new(font_data);
	let transform = Mat4::from_scale(Vec3::new(pixel_size, pixel_size, 0.)).to_cols_array();
	let text_mesh: MeshText = generator
		.generate_section(text, true, Some(&transform))
		.unwrap();

	let vertices = text_mesh.vertices;
	let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
	let uvs = vec![[0f32, 0f32]; positions.len()];

	let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
	mesh.compute_flat_normals();

	(mesh, text_mesh.bbox)
}

pub trait BoundingBoxExt {
	fn get_required_text_offset(self) -> Vec3;
}

impl BoundingBoxExt for meshtext::BoundingBox {
	fn get_required_text_offset(self) -> Vec3 {
		Vec3::X * (self.size().x / -2.) + Vec3::Y * (self.size().y / -2.)
	}
}

/// Returns mesh + offset (to ensure coordinates start in center of text).
/// Without taking offset into account, text will be rendered with *top right* corner at center of entity.
pub fn get_text_mesh(text: &str, pixel_size: f32) -> (Mesh, Vec3) {
	let (mesh, bbox) = get_text_mesh_with_bbox(text, pixel_size);
	(mesh, bbox.get_required_text_offset())
}
