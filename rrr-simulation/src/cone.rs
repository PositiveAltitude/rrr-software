use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};


#[derive(Clone, Copy, Debug)]
pub struct Cone {
    pub radius: f32,
    pub height: f32,
    pub resolution: u32,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            radius: 0.5,
            height: 1.0,
            resolution: 16,
        }
    }
}

impl From<Cone> for Mesh {
    fn from(c: Cone) -> Self {
        debug_assert!(c.radius > 0.0);
        debug_assert!(c.height > 0.0);
        debug_assert!(c.resolution > 2);

        let num_vertices = c.resolution + 2;
        let num_polygons = c.resolution * 2;
        let num_indices = num_polygons * 3;

        let mut positions = Vec::with_capacity(num_vertices as usize);
        let mut normals = Vec::with_capacity(num_vertices as usize);
        let mut indices = Vec::with_capacity(num_indices as usize);

        let step_theta = std::f32::consts::TAU / c.resolution as f32;

        let center_vertex_index = num_vertices - 2;
        let top_vertex_index = num_vertices - 1;

        for i in 0..c.resolution {
            let theta = i as f32 * step_theta;
            let (sin, cos) = theta.sin_cos();

            positions.push([cos * c.radius, sin * c.radius, 0.0]);
            normals.push([cos, sin, 0.0]);
        }

        positions.push([0.0, 0.0, 0.0]);
        positions.push([0.0, 0.0, c.height]);
        normals.push([0.0, 0.0, -1.0]);
        normals.push([0.0, 0.0, 1.0]);

        for i in 1..c.resolution {
            indices.extend_from_slice(&[center_vertex_index, i, i - 1]);
            indices.extend_from_slice(&[top_vertex_index, i - 1, i]);
        }
        indices.extend_from_slice(&[center_vertex_index, 0, c.resolution - 1]);
        indices.extend_from_slice(&[top_vertex_index, c.resolution - 1, 0]);


        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    }
}