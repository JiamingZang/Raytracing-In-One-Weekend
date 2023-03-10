use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::rand_01,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, Color},
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

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
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
            albedo,
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
        *scattered = Ray::new(
            rec.p,
            reflected + random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = if dot(&-unit_direction, &rec.normal) > 1.0 {
            1.0
        } else {
            dot(&-unit_direction, &rec.normal)
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_reflect || reflectance(cos_theta, refraction_ratio) > rand_01() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };
        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
