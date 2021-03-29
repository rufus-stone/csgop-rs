/*
constexpr auto earth_radius = 6378137.0;
constexpr auto origin_shift = (2.0 * M_PI * earth_radius) / 2.0;
constexpr auto tile_size = 256;
constexpr auto initial_resolution = (2.0 * M_PI * earth_radius) / tile_size; // 156543.03392804062 for a tile_size of 256
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }

    pub fn from_str(xyz: &str) -> Point {
        Point { x: 0, y: 0, z: 0 }
    }

    pub fn from_xyz(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    pub fn to_decimal_degrees(&self) -> String {
        "".to_owned()

        // xy_to_lat_lon()
    }

    pub fn xy_to_lat_long(xy: &Point, map_origin: &Point, map_scale: f32) -> Point {
        // Convert the CS:GO grid positions to X/Y pixel positions on a 1024x1024 image

        // According to https://www.maptiler.com/google-maps-coordinates-tile-bounds-projection/
        // ... at zoom level 2 using Spherical Mercator projection, the map will be 1024x1024 pixels, which is the same size as our game map images.
        // The EPSG projection caps latitude values at 85.0511287798066 and -85.05112877980659, as the projection rapidly tends towards infinity beyond this point.

        Point::new()
    }

    pub fn pixels_to_metres(pixels_xy: &Point, zoom_level: u8) -> Point {
        Point::new()
    }
}

pub fn game_pos_to_decimal_degrees(game_coordinates: &str) -> String {
    // First, convert the game coordinates string into a Point representing a CS:GO grid position

    // Second, convert the CS:GO grid position to a Point representing the corresponding X/Y pixel position on a 1024x1024 image of the map being played

    // According to https://www.maptiler.com/google-maps-coordinates-tile-bounds-projection/
    // ... at zoom level 2 using Spherical Mercator projection, the map will be 1024x1024 pixels, which is the same size as our game map images.
    // The EPSG projection caps latitude values at 85.0511287798066 and -85.05112877980659, as the projection rapidly tends towards infinity beyond this point.

    "".to_owned()
}

pub fn metres_between_points(p1: &str, p2: &str) -> String {
    "".to_owned()
}
