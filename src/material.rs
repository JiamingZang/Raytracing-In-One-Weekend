use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, Color},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::ray(rec.p, scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: {
                if f < 1.0 {
                    f
                } else {
                    1.0
                }
            },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        *scattered = Ray::ray(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}
