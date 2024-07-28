use crate::buffer::Rgb;
use crate::vector_math::Vec3;

pub fn create_scene() -> Scene {
    Scene {
        objects: vec![
            Object {
                surface: Surface::Sphere(Sphere {
                    origin: Vec3::new(1.7, 1.8, -10.0),
                    radius: 2.0,
                }),
                material: &RUBBERY_RED,
            },
            Object {
                surface: Surface::Sphere(Sphere {
                    origin: Vec3::new(2.5, 0.8, -7.0),
                    radius: 1.0,
                }),
                material: &GLOSSY_GREEN,
            },
            Object {
                surface: Surface::Sphere(Sphere {
                    origin: Vec3::new(-2.5, 0.2, -5.0),
                    radius: 1.0,
                }),
                material: &GLOSSY_BLUE,
            },
            Object {
                surface: Surface::Sphere(Sphere {
                    origin: Vec3::new(4.5, 5.2, -11.0),
                    radius: 2.5,
                }),
                material: &MIRROR,
            },
            Object {
                surface: Surface::Sphere(Sphere {
                    origin: Vec3::new(-1.2, -0.6, -4.0),
                    radius: 0.7,
                }),
                material: &GLASS,
            },
        ],
        lights: vec![
            Light {
                origin: Vec3::new(10.0, 14.0, 10.0),
                intensity: 0.4,
            },
            Light {
                origin: Vec3::new(5.0, 0.5, -4.0),
                intensity: 0.8,
            },
        ],
    }
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
}

impl Sphere {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<(f32, Vec3)> {
        let ray_direction = ray_direction.normalize();
        let ray_origin_to_sphere = self.origin - *ray_origin;
        let center_to_ray_square = ray_origin_to_sphere.magnitude().powi(2)
            - (ray_origin_to_sphere * ray_direction).powi(2);
        if center_to_ray_square <= self.radius.powi(2) {
            let distance_to_intersection_with_ray = ray_origin_to_sphere * ray_direction;
            let delta = (self.radius.powi(2) - center_to_ray_square).sqrt();
            if distance_to_intersection_with_ray - delta >= 0.0 {
                let distance = distance_to_intersection_with_ray - delta;
                return Some((
                    distance,
                    ray_direction * distance + *ray_origin - self.origin,
                ));
            } else if distance_to_intersection_with_ray + delta >= 0.0 {
                let distance = distance_to_intersection_with_ray + delta;
                return Some((
                    distance,
                    ray_direction * distance + *ray_origin - self.origin,
                ));
            }
        }

        Option::None
    }
}
#[derive(Debug, PartialEq)]
pub enum Surface {
    Sphere(Sphere),
    Triange(),
}

impl Surface {
    pub fn find_intersection(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Option<(f32, Vec3)> {
        match self {
            Self::Sphere(sphere) => sphere.find_intersection(ray_origin, ray_direction),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub material: &'static Material,
    pub surface: Surface,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub diffuse_color: Rgb,
    pub shininess: f32,
    pub albedo: (f32, f32, f32, f32), // diffuse, specular, reflection
    pub refractive_index: f32,
}
pub struct Light {
    pub origin: Vec3,
    pub intensity: f32,
}

static GLOSSY_BLUE: Material = Material {
    diffuse_color: Rgb::new(50, 50, 170),
    shininess: 200.0,
    albedo: (0.5, 0.5, 0.0, 0.0),
    refractive_index: 0.0,
};

static RUBBERY_RED: Material = Material {
    diffuse_color: Rgb::new(190, 30, 30),
    shininess: 10.0,
    albedo: (0.9, 0.1, 0.0, 0.0),
    refractive_index: 0.0,
};

static GLOSSY_GREEN: Material = Material {
    diffuse_color: Rgb::new(50, 250, 50),
    shininess: 50.0,
    albedo: (0.8, 0.6, 0.1, 0.0),
    refractive_index: 0.0,
};

static MIRROR: Material = Material {
    diffuse_color: Rgb::new(10, 10, 10),
    shininess: 200.0,
    albedo: (0.2, 0.6, 0.8, 0.0),
    refractive_index: 0.0,
};

static GLASS: Material = Material {
    diffuse_color: Rgb::new(10, 10, 10),
    shininess: 200.0,
    albedo: (0.0, 0.6, 0.0, 0.9),
    refractive_index: 1.8,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_intersects_test() {
        let sphere = Sphere {
            origin: Vec3::new(3.0, 0.0, 0.0),
            radius: 2.0,
        };

        let (distance, _) = sphere
            .find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
            .unwrap();

        assert_eq!(distance, 1.0);
    }

    #[test]
    fn ray_intersects_with_sphere_behind_camera() {
        let sphere = Sphere {
            origin: Vec3::new(-3.0, 0.0, 0.0),
            radius: 2.0,
        };

        let result = sphere.find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        assert_eq!(result, None);
    }

    #[test]
    fn ray_intersects_with_camera_inside_sphere() {
        let sphere = Sphere {
            origin: Vec3::new(-1.0, 0.0, 0.0),
            radius: 2.0,
        };

        let (distance, _) = sphere
            .find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
            .unwrap();

        assert_eq!(distance, 1.0);
    }
}
