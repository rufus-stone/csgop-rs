/*
constexpr auto earth_radius = 6378137.0;
constexpr auto origin_shift = (2.0 * M_PI * earth_radius) / 2.0;
constexpr auto tile_size = 256;
constexpr auto initial_resolution = (2.0 * M_PI * earth_radius) / tile_size; // 156543.03392804062 for a tile_size of 256
*/

use serde::{Deserialize, Serialize};

const MAP_DIMENSIONS: &str = r#"({"maps":{"ar_baggage":{"pos_x":-2150,"pos_y":2280,"scale":4.0},"ar_dizzy":{"pos_x":-2512,"pos_y":1536,"scale":3.0},"ar_monastery":{"pos_x":-1687,"pos_y":1640,"scale":3.0},"ar_shoots":{"pos_x":-2150,"pos_y":2582,"scale":4},"coop_cementplant":{"pos_x":-5160,"pos_y":5859,"scale":12},"coop_kasbah":{"pos_x":-5160,"pos_y":5859,"scale":12},"cs_agency":{"pos_x":-2947,"pos_y":2492,"scale":5},"cs_assault":{"pos_x":4041,"pos_y":7838,"scale":4.6},"cs_backalley":{"pos_x":-2485,"pos_y":3200,"scale":3.50},"cs_insertion":{"pos_x":-4888,"pos_y":4884,"scale":10},"cs_italy":{"pos_x":-2647,"pos_y":2592,"scale":4.6},"cs_militia":{"pos_x":-1474,"pos_y":2296,"scale":4.5},"cs_office":{"pos_x":-1838,"pos_y":1858,"scale":4.1},"cs_rush":{"pos_x":-2950,"pos_y":3350,"scale":5.2},"cs_siege":{"pos_x":-1193,"pos_y":3515,"scale":6.5},"cs_workout":{"pos_x":-2176,"pos_y":3165,"scale":6.06},"de_abbey":{"pos_x":-6204,"pos_y":5111,"scale":6.5},"de_ali":{"pos_x":-2064,"pos_y":2920,"scale":5},"de_anubis":{"pos_x":-2796,"pos_y":3328,"scale":5.22},"de_austria":{"pos_x":-2877,"pos_y":2930,"scale":5.80},"de_aztec":{"pos_x":-3200,"pos_y":2841,"scale":6},"de_bazaar":{"pos_x":-2434,"pos_y":2179,"scale":5.0},"de_biome":{"pos_x":-2129,"pos_y":2368,"scale":5.00},"de_blackgold":{"pos_x":-1100,"pos_y":1425,"scale":5.30},"de_breach":{"pos_x":-2950,"pos_y":2886,"scale":5.5},"de_cache":{"pos_x":-2000,"pos_y":3250,"scale":5.5},"de_canals":{"pos_x":-2496,"pos_y":1792,"scale":4},"de_castle":{"pos_x":-3378,"pos_y":2756,"scale":5.5},"de_chinatown":{"pos_x":-1735,"pos_y":3232,"scale":4},"de_chlorine":{"pos_x":2076,"pos_y":1272,"scale":5.25},"de_coast":{"pos_x":-3028,"pos_y":4122,"scale":5.50},"de_dust2":{"pos_x":-2476,"pos_y":3239,"scale":4.4},"de_dust":{"pos_x":-2850,"pos_y":4073,"scale":6},"de_empire":{"pos_x":-2165,"pos_y":2000,"scale":4.5},"de_facade":{"pos_x":-90,"pos_y":5659,"scale":6},"de_gwalior":{"pos_x":-1145,"pos_y":2688,"scale":5},"de_inferno":{"pos_x":-2087,"pos_y":3870,"scale":4.9},"de_lite":{"pos_x":-2012,"pos_y":2928,"scale":5},"de_log":{"pos_x":-411,"pos_y":759,"scale":6.50},"de_marquis":{"pos_x":-1877,"pos_y":3199,"scale":5.0},"de_mikla":{"pos_x":711,"pos_y":2383,"scale":4.1},"de_mirage":{"pos_x":-3230,"pos_y":1713,"scale":5.00},"de_mist":{"pos_x":-5150,"pos_y":2080,"scale":4.8},"de_nuke":{"pos_x":-3453,"pos_y":2887,"scale":7},"de_overgrown_b7":{"pos_x":-3376,"pos_y":5563,"scale":7},"de_overpass":{"pos_x":-4831,"pos_y":1781,"scale":5.2},"Rails":{"pos_x":-2199,"pos_y":2874,"scale":4.5},"de_resort":{"pos_x":-506,"pos_y":2713,"scale":5.5},"de_royal":{"pos_x":-2343,"pos_y":2644,"scale":4},"de_ruby":{"pos_x":-1079,"pos_y":3093,"scale":4.50},"de_ruins":{"pos_x":-2443,"pos_y":2485,"scale":6.25},"de_safehouse":{"pos_x":-240,"pos_y":2650,"scale":4.52},"Santorini":{"pos_x":-2135,"pos_y":1400,"scale":4},"de_seaside":{"pos_x":-4161,"pos_y":3680,"scale":7},"de_season":{"pos_x":-1003,"pos_y":2521,"scale":5.00},"de_shipped":{"pos_x":-2432,"pos_y":2663,"scale":5.80},"de_shortdust":{"pos_x":-2318,"pos_y":2337,"scale":3.6},"de_shortnuke":{"pos_x":-3453,"pos_y":2887,"scale":7},"de_shorttrain":{"pos_x":-2477,"pos_y":2392,"scale":4.7},"de_stmarc":{"pos_x":-9383,"pos_y":9099,"scale":4},"de_studio":{"pos_x":-3248,"pos_y":2968,"scale":6.17},"de_subzero":{"pos_x":-2438,"pos_y":3690,"scale":5.0},"de_sugarcane":{"pos_x":-4015,"pos_y":2000,"scale":4.25},"de_thrill":{"pos_x":-3276,"pos_y":2973,"scale":5.5},"de_train":{"pos_x":-2477,"pos_y":2392,"scale":4.7},"de_tulip":{"pos_x":3402,"pos_y":5583,"scale":5.50},"de_vertigo":{"pos_x":-3168,"pos_y":1762,"scale":4.0},"de_zoo":{"pos_x":-2435,"pos_y":6116,"scale":7},"dz_blacksite":{"pos_x":-8604,"pos_y":8804,"scale":17.0},"dz_junglety":{"pos_x":-8504,"pos_y":8741,"scale":17.0},"dz_sirocco":{"pos_x":-8604,"pos_y":8804,"scale":17.0},"de_bank":{"pos_x":-2000,"pos_y":1493,"scale":4},"de_cbble":{"pos_x":-3840,"pos_y":3072,"scale":6},"gd_crashsite":{"pos_x":-2212,"pos_y":1437,"scale":3.5},"de_lake":{"pos_x":1200,"pos_y":-700,"scale":5.2},"ar_lunacy":{"pos_x":-1536,"pos_y":1536,"scale":3.0},"gd_rialto":{"pos_x":-1260,"pos_y":1836,"scale":3.0},"training1":{"pos_x":-2510,"pos_y":2000,"scale":5}}})"#;

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
