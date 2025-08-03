use macroquad::{color::WHITE, math::Vec2, models::Mesh, ui::Vertex};

use super::wire_variant::WireVariant;

pub struct WireMesh {
    pub mesh: Mesh,
}

pub fn generate_wire_meshes() -> Vec<WireMesh> {
    let mut wire_meshes: Vec<WireMesh> = Vec::new();

    for variant in 0..32 {
        let mesh = generate_wire_mesh(WireVariant(variant));
        wire_meshes.push(WireMesh { mesh });
    }
    wire_meshes
}
fn generate_wire_mesh(variant: WireVariant) -> Mesh {
        let mut vertices = vec![];
    let mut indices = vec![];

    // draw center square
    let s = 0.2;
    let off = (1.0 - s) / 2.0;
    append_rect(&mut vertices, &mut indices, off, off, s, s);

    if variant.has_north() {
        append_rect(&mut vertices, &mut indices, off, 0.0, s, 0.5);
    }
    if variant.has_south() {
        append_rect(&mut vertices, &mut indices, off, 0.5, s, 0.5);
    }
    if variant.has_west() {
        append_rect(&mut vertices, &mut indices, 0.0, off, 0.5, s);
    }
    if variant.has_east() {
        append_rect(&mut vertices, &mut indices, 0.5, off, 0.5, s);
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

fn append_rect(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let base = vertices.len() as u16;

    vertices.push(Vertex::new(x, y, 0.0, 0.0, 0.0, WHITE));
    vertices.push(Vertex::new(x + w, y, 0.0, 0.0, 0.0, WHITE));
    vertices.push(Vertex::new(x + w, y + h, 0.0, 0.0, 0.0, WHITE));
    vertices.push(Vertex::new(x, y + h, 0.0, 0.0, 0.0, WHITE));

    indices.extend_from_slice(&[
        base, base + 1, base + 2,
        base, base + 2, base + 3,
    ]);
}

pub fn apply_transform(original: &WireMesh, transform: Vec2) -> WireMesh {
    let mut ret = WireMesh {
        mesh: Mesh {
            vertices: original.mesh.vertices.clone(),
            indices: original.mesh.indices.clone(),
            texture: None,
        }
    };

    for v in ret.mesh.vertices.iter_mut() {
        v.position.x += transform.x;
        v.position.y += transform.y;
    }

    ret
}
