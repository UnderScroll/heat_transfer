use fixed::{FixedI64, types::extra::U16};
use nannou::{Draw, color::GREEN, geom::Point2, math::clamp};

use crate::{array_2d::Array2D, color_gradient, material::Material};

pub struct Simulation {
    parameters: Parameters,
    heat_flux: Array2D<HeatFlux>,
    cells: Array2D<Cell>,
}

pub struct Parameters {
    /// Update time step (`s`)
    pub time_step: f32,
    /// Space step (`m`)
    pub space_step: f32,
    /// Amount of cells in the horizontal direction
    pub width: usize,
    /// Amount of cells in the vertical direction
    pub height: usize,
}

#[allow(dead_code)]
impl Simulation {
    pub fn new(cell: Cell, parameters: Parameters) -> Self {
        let heat_flux = Array2D::new_with_item(
            HeatFlux {
                vertical: FixedI64::ZERO,
                horizontal: FixedI64::ZERO,
            },
            parameters.width,
            parameters.height,
        );
        let cells = Array2D::new_with_item(cell, parameters.width, parameters.height);

        Self {
            parameters,
            heat_flux,
            cells,
        }
    }

    pub fn from_material_grid(grid: Vec<Vec<&'static Material>>, temperature: f32, parameters: Parameters) -> Self {
        let width = grid.len();
        let height = grid[0].len();

        let cells_materials = grid.into_iter().flatten();
        let cells: Vec<Cell> = cells_materials
            .map(|material| Cell::with_temperature(material, parameters.space_step, temperature))
            .collect();

        let cells = Array2D::new(cells, width, height);

        let heat_flux = Array2D::new_with_item(
            HeatFlux {
                vertical: FixedI64::ZERO,
                horizontal: FixedI64::ZERO,
            },
            parameters.width,
            parameters.height,
        );
        Self {
            parameters,
            heat_flux,
            cells,
        }
    }

    fn update_heat_flux(&mut self) {
        // Aliases
        let (width, height) = (self.parameters.width, self.parameters.height);
        let time_step = self.parameters.time_step;

        // Heat flux computation (except last line horizontal heat flux)
        let mut line_index = 0;
        let mut next_line_index = width;
        for _y in 0..height - 1 {
            let heat_flux_line = &mut self.heat_flux.data[line_index..next_line_index];
            let two_cell_lines = &self.cells.data[line_index..next_line_index + width];

            // Compute vertical and horizontal heat flux (except last vertical heat flux)
            for x in 0..width - 1 {
                // Adjacent cells
                let cell = &two_cell_lines[x];
                let right_cell = &two_cell_lines[x + 1];
                let bottom_cell = &two_cell_lines[x + width];

                // Compute heat flux between the cells
                heat_flux_line[x] = HeatFlux {
                    horizontal: cell.heat_flux(right_cell, time_step),
                    vertical: cell.heat_flux(bottom_cell, time_step),
                };
            }

            // Compute last vertical heat flux
            let cell = &two_cell_lines[width - 1];
            let bottom_cell = &two_cell_lines[width + width - 1];

            heat_flux_line[width - 1].vertical = cell.heat_flux(bottom_cell, time_step);

            // Next indexes
            line_index = next_line_index;
            next_line_index += width;
        }

        // Compute last line horizontal heat flux
        let heat_flux_line = &mut self.heat_flux.data[line_index..];
        let cell_line = &self.cells.data[line_index..];
        for x in 0..width - 1 {
            // Adjacent cells
            let cell = &cell_line[x];
            let right_cell = &cell_line[x + 1];

            // Compute heat flux between the cells
            heat_flux_line[x].horizontal = cell.heat_flux(right_cell, time_step);
        }
    }

    fn update_energy(&mut self) {
        // Aliases
        let (width, height) = (self.parameters.width, self.parameters.height);

        // Update
        let mut line_index = 0;
        for _y in 0..height - 1 {
            let cells_two_lines = &mut self.cells.data[line_index..line_index + width * 2];
            let heat_flux_line = &self.heat_flux.data[line_index..line_index + width];

            // Apply energy transfer
            for x in 0..width {
                let heat_flux = heat_flux_line[x];

                // Note: The bottom row is not updated
                cells_two_lines[x].add_energy(heat_flux.horizontal + heat_flux.vertical);
                cells_two_lines[x + 1].add_energy(-heat_flux.horizontal);
                cells_two_lines[x + width].add_energy(-heat_flux.vertical);
            }

            // Update line index
            line_index += width;
        }

        // Bottom row horizontal
        let last_line_index = height * width - width;
        let cells_line = &mut self.cells.data[last_line_index..];
        let heat_flux_line = &self.heat_flux.data[last_line_index..];
        for x in 0..width - 1 {
            let heat_flux = heat_flux_line[x].horizontal;

            cells_line[x].add_energy(heat_flux);
            cells_line[x + 1].add_energy(-heat_flux);
        }
    }

    pub fn update(&mut self) {
        self.update_heat_flux();
        self.update_energy();
    }

    pub fn draw(&self, draw: &Draw, window_width: f32, window_height: f32) {
        let width = self.parameters.width;
        let height = self.parameters.height;

        let max_cell_width = window_width / self.cells.width as f32;
        let max_cell_height = window_height / self.cells.height as f32;
        let size = max_cell_width.min(max_cell_height);

        let mut i = 0;
        for y in 0..height {
            for x in 0..width {
                let cell = &self.cells.data[i];

                let (x, y) = (x as f32, y as f32);

                let color =
                    color_gradient::sample_f32(clamp::<f32>((cell.temperature - 273.15) / (1000.0 - 273.15), 0.0, 1.0));

                let h_offset = size * (self.cells.width as f32 * 0.5 - 0.5);
                let v_offset = size * (self.cells.height as f32 * 0.5 - 0.5);
                let position = Point2::new(x * size, y * size) - Point2::new(h_offset, v_offset);

                draw.rect().xy(position).w_h(size, size).color(color).finish();

                if size > 70.0 {
                    draw.text(&format!("{:.1}°C", cell.temperature - 273.15))
                        .xy(position)
                        .w_h(size, size)
                        .font_size(20)
                        .color(GREEN)
                        .finish();
                }

                i += 1;
            }
        }
    }

    #[inline]
    pub fn set_temperature(&mut self, temperature: f32, x: usize, y: usize) {
        self.cells.get_mut(x, y).set_temperature(temperature);
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            time_step: 0.01,  // 10ms,
            space_step: 0.02, // 20mm
            width: 10,
            height: 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HeatFlux {
    horizontal: FixedI64<U16>,
    vertical: FixedI64<U16>,
}

/// Unit of space in the simulation
/// Assumed to be a cube
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    /// Material of the cell
    material: &'static Material,
    /// Length of the cell
    length: f32,
    /// Mass of the cell (unit: kg)
    mass: f32,
    /// Amount of energy in the cell (unit: J) - Exact
    ///
    /// Note: This uses a signed value even though the energy is always positive,
    /// this is to be able to perform operation with variation of energy that can be negative
    energy: FixedI64<U16>,
    /// Temprature of the cell (unit: K) - Approximate
    temperature: f32,
}

#[allow(dead_code)]
impl Cell {
    #[must_use]
    pub fn with_temperature(material: &'static Material, size: f32, temperature: f32) -> Self {
        let volume = size * size * size;
        let mass = material.density * volume;

        let energy = temperature_to_energy(temperature, mass, material.specific_heat_capacity);

        Self {
            material,
            mass,
            energy,
            temperature,
            length: size,
        }
    }

    /// Sets the temperature of the cell to the given value (`K`).
    /// Updates the energy required to attain that temperature.
    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
        self.energy = temperature_to_energy(temperature, self.mass, self.material.specific_heat_capacity);
    }

    /// Sets the energy of the cell to the given value (`J`).
    /// Updates the temperature accordingly.
    pub fn set_energy(&mut self, energy: FixedI64<U16>) {
        self.energy = energy;
        self.temperature = energy_to_temperature(energy, self.mass, self.material.specific_heat_capacity);
    }

    /// Add given energy value (`J`) to the cell.
    /// Updates the temperature accordingly.
    pub fn add_energy(&mut self, energy: FixedI64<U16>) {
        self.energy += energy;
        self.temperature = energy_to_temperature(self.energy, self.mass, self.material.specific_heat_capacity);
    }

    /// Computes the heat flux between two cells.
    /// It describes how much energy of `self` is transfered to (or recived from depending on sign) the `other` cell in a given `time_step` (`s`)
    /// This function is **not** commutative.
    pub fn heat_flux(&self, other: &Cell, time_step: f32) -> FixedI64<U16> {
        // Effective condictivity (harmonic mean)
        let effective_conductivity =
            harmonic_mean(self.material.thermal_conductivity, other.material.thermal_conductivity);

        // Heat flux computation
        let temperature_diff = self.temperature - other.temperature;
        let heat_flux = -effective_conductivity * temperature_diff * time_step * self.length;

        FixedI64::from_num(heat_flux)
    }
}

/// Compute the temperature of a material containing a given amount of energy (`J`) of a given a mass (`kg`) and a specific heat capacity (`J/(kg.K)`)
pub fn energy_to_temperature(energy: FixedI64<U16>, mass: f32, specific_heat_capacity: f32) -> f32 {
    energy.to_num::<f32>() / (specific_heat_capacity * mass)
}

/// Compute the energy contained by a meterial of a given a mass (`kg`) and a specific heat capacity (`J/(kg.K)`) at a given temperature (`K`)
pub fn temperature_to_energy(temperature: f32, mass: f32, specific_heat_capacity: f32) -> FixedI64<U16> {
    FixedI64::from_num(specific_heat_capacity * mass * temperature)
}

/* Utility functions */
fn harmonic_mean(a: f32, b: f32) -> f32 {
    (2.0 * a * b) / (a + b)
}
