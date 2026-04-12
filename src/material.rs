#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Material {
    /// Thermal Conductivity: W/(m.K)
    pub thermal_conductivity: f32,
    /// Specific Heat Capacity: `J/(kg.K)`
    pub specific_heat_capacity: f32,
    /// Density: `kg/m³`
    pub density: f32,
    /// Thermal Diffusivity: (m²/s)
    pub thermal_diffusivity: f32,
}

impl Material {
    const fn new(thermal_conductivity: f32, specific_heat_capacity: f32, density: f32) -> Self {
        Self {
            thermal_conductivity,
            specific_heat_capacity,
            density,
            thermal_diffusivity: thermal_conductivity / (density * specific_heat_capacity),
        }
    }
}

pub static VOID: Material = Material::new(1e-16, 1e-16, 1e-16); // None zero to avoid zero division
pub static AIR: Material = Material::new(0.025, 1012.0, 1.29);
pub const ALUMINUM: Material = Material::new(225.94, 896.894, 2699.0);
pub const COPPER: Material = Material::new(397.48, 384.603, 8940.0);
pub const WATER: Material = Material::new(0.6065, 4184.5034, 1000.0);
