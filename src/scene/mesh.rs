use super::triangle::Triangle;
use super::Surface;

use crate::ray_tracing::Intersection;
use crate::vector_math::Vec3;

#[derive(Debug, PartialEq)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Surface for Mesh {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersection> {
        let mut closest_distance = f32::MAX;
        let mut closest_traingle = None;
        let mut intersection_coords = None;
        for triangle in &self.triangles {
            if let Some(Intersection {
                distance,
                texture_coords,
                ..
            }) = triangle.find_intersection(ray_origin, ray_direction)
            {
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_traingle = Some(triangle);
                    intersection_coords = texture_coords;
                }
            }
        }
        closest_traingle.map(|triangle| Intersection {
            distance: closest_distance,
            normal: triangle.normal(),
            texture_coords: intersection_coords,
        })
    }
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh { triangles }
    }

    pub fn from_obj_file(file_name: &str) -> Mesh {
        let (models, _) = tobj::load_obj(file_name, &tobj::LoadOptions::default())
            .expect(&format!("Can not load file {}", file_name));

        let model = &models[0];

        let mut triangles = vec![];

        let number_of_triangles = model.mesh.indices.len() / 3;

        let indices: Vec<_> = model.mesh.indices.iter().map(|idx| *idx as usize).collect();
        let positions = &model.mesh.positions;

        for face_idx in 0..number_of_triangles {
            let offset = face_idx * 3;

            triangles.push(Triangle::new(
                Vec3::new(
                    positions[indices[offset] * 3] as f32,
                    positions[indices[offset] * 3 + 1] as f32,
                    positions[indices[offset] * 3 + 2] as f32,
                ),
                Vec3::new(
                    positions[indices[offset + 1] * 3] as f32,
                    positions[indices[offset + 1] * 3 + 1] as f32,
                    positions[indices[offset + 1] * 3 + 2] as f32,
                ),
                Vec3::new(
                    positions[indices[offset + 2] * 3] as f32,
                    positions[indices[offset + 2] * 3 + 1] as f32,
                    positions[indices[offset + 2] * 3 + 2] as f32,
                ),
            ));
        }

        Mesh { triangles }
    }
}
