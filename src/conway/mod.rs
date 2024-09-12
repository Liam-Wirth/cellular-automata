pub mod conway_map;

pub const DEFAULT_MAP_SIZE: i32 = 75;
pub const DEFAULT_CELL_SIZE: f32 = 10.0;

/// "Neighbor" cells around the current cell, coordinates are organized in standard x,y format
/// ## Think of the layout like this:
/// (-1,1 ) (0,1 )  (1,1 )
/// (-1,0 ) (cell)  (1,0 )
/// (-1,-1) (0,-1)  (1,-1)
const NEIGHBORS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
