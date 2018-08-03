extern crate noise;
extern crate rand;

pub mod tile;

use self::noise::{NoiseFn, Point2, Seedable};
use super::*;
use tile::Tile;

pub struct Terrain {
    tiles: Vec<Vec<Tile>>,
}

impl Terrain {
    pub fn update_all(&mut self, time: f64, climate: &Climate) {
        for column in &mut self.tiles {
            for tile in column {
                tile.update(time, climate);
            }
        }
    }

    /// Gets a mutable reference to that tile, this function should be used as little as possible.
    pub fn get_tile_at_mut(&mut self, pos: BoardCoordinate) -> &mut Tile {
        let (x, y) = pos;

        assert!(
            x < self.get_width(),
            "There is no `Tile` at the given x coordinate: {}.",
            x
        );
        assert!(
            y < self.get_height(),
            "There is no `Tile` at the given y coordinate: {}.",
            y
        );

        return &mut self.tiles[x][y];
    }

    /// Gets a reference to that tile, this function should be used as little as possible.
    ///
    /// Usage of this function is however encouraged inside the `terrain` module.
    pub fn get_tile_at(&self, pos: BoardCoordinate) -> &Tile {
        let (x, y) = pos;

        assert!(
            x < self.get_width(),
            "There is no `Tile` at the given x coordinate: {}.",
            x
        );
        assert!(
            y < self.get_height(),
            "There is no `Tile` at the given y coordinate: {}.",
            y
        );

        return &self.tiles[x][y];
    }

    pub fn update_at(&mut self, pos: BoardCoordinate, time: f64, climate: &Climate) {
        let (x, y) = pos;
        self.tiles[x][y].update(time, climate);
    }

    pub fn generate_perlin(board_size: BoardSize, step_size: f64) -> Self {
        let noise_generator = noise::Perlin::new();

        // Seed the noise generator.
        let noise_generator = noise_generator.set_seed(rand::random());

        return Terrain::generate_terrain_with_noise(noise_generator, board_size, step_size);
    }

    /// Tries to add `food` to the food level of that `Tile`.
    ///
    /// Does nothing for water tiles.
    pub fn add_food_or_nothing_at(&mut self, pos: BoardCoordinate, food: f64) {
        self.tiles[pos.0][pos.1].add_food_or_nothing(food);
    }

    fn generate_terrain_with_noise<N: NoiseFn<Point2<f64>>>(
        ng: N,
        board_size: BoardSize,
        step_size: f64,
    ) -> Self {
        let (board_width, board_height) = board_size;

        let mut tiles = Vec::with_capacity(board_width);

        // allocate these variables
        let mut big_force: f64;
        let mut fertility: f64;
        let mut climate_type: f64;
        for x in 0..board_width {
            tiles.push(Vec::with_capacity(board_height));
            for y in 0..board_height {
                big_force = (y as f64 / board_height as f64).sqrt();

                // TODO: understand these formulas.
                fertility = ng
                    .get([x as f64 * step_size * 3.0, y as f64 * step_size * 3.0])
                    .abs()
                    * (1.0 - big_force)
                    * 5.0
                    + ng.get([x as f64 * step_size * 0.5, y as f64 * step_size * 0.5])
                        .abs()
                        * big_force
                        * 5.0
                    - 1.5;

                climate_type = ng
                    .get([
                        x as f64 * step_size * 0.2 + 10000.0,
                        y as f64 * step_size * 0.2 + 10000.0,
                    ]).abs()
                    * 1.63
                    - 0.4;

                climate_type = climate_type.max(0.0).min(0.8);
                tiles[x].push(Tile::new(fertility, climate_type));
            }
        }

        // Return the generated terrain.
        Terrain { tiles }
    }

    pub fn get_width(&self) -> usize {
        return self.tiles.len();
    }

    pub fn get_height(&self) -> usize {
        return self.tiles[0].len();
    }
}
