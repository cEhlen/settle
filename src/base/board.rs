use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Hash)]
pub struct Intersection(u8, u8, u8);

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Intersection {}

#[derive(Debug)]
pub struct RoadPosition(u8, u8);




pub const ALL_INTERSECTIONS: [Intersection; 54] = [
    // Row 1
    Intersection(0, 4, 5),
    Intersection(0, 1, 5),
    Intersection(1, 5, 6),
    Intersection(1, 2, 6),
    Intersection(2, 6, 7),
    Intersection(2, 3, 7),
    Intersection(3, 7, 8),
    // Row 2
    Intersection(4, 9, 10),
    Intersection(4, 5, 10),
    Intersection(5, 10, 11),
    Intersection(5, 6, 11),
    Intersection(6, 11, 12),
    Intersection(6, 7, 12),
    Intersection(7, 12, 13),
    Intersection(7, 8, 13),
    Intersection(8, 13, 14),
    // Row 3
    Intersection(9, 15, 16),
    Intersection(9, 10, 16),
    Intersection(10, 16, 17),
    Intersection(10, 11, 17),
    Intersection(11, 17, 18),
    Intersection(11, 12, 18),
    Intersection(12, 18, 19),
    Intersection(12, 13, 19),
    Intersection(13, 19, 20),
    Intersection(13, 14, 20),
    Intersection(14, 20, 21),
    // Row 4
    Intersection(15, 16, 22),
    Intersection(16, 22, 23),
    Intersection(16, 17, 23),
    Intersection(17, 23, 24),
    Intersection(17, 18, 24),
    Intersection(18, 24, 25),
    Intersection(18, 19, 25),
    Intersection(19, 25, 26),
    Intersection(19, 20, 26),
    Intersection(20, 26, 27),
    Intersection(20, 21, 27),
    // Row 5
    Intersection(22, 23, 28),
    Intersection(23, 28, 29),
    Intersection(23, 24, 29),
    Intersection(24, 29, 30),
    Intersection(24, 25, 30),
    Intersection(25, 30, 31),
    Intersection(25, 26, 31),
    Intersection(26, 31, 32),
    Intersection(26, 27, 32),
    // Row 6
    Intersection(28, 29, 33),
    Intersection(29, 33, 34),
    Intersection(29, 30, 34),
    Intersection(30, 34, 35),
    Intersection(30, 31, 35),
    Intersection(31, 35, 36),
    Intersection(31, 32, 36),
];

lazy_static! {
pub static ref INTERSECTION_CONNECTIONS: HashMap<Intersection, Vec<u8>> = HashMap::from([
    // Row 1
    (Intersection(0, 4, 5), vec![1, 8]),
    (Intersection(0, 1, 5), vec![0, 2]),
    (Intersection(1, 5, 6), vec![1, 3, 10]),
    (Intersection(1, 2, 6), vec![2, 4]),
    (Intersection(2, 6, 7), vec![3, 5, 12]),
    (Intersection(2, 3, 7), vec![4, 6]),
    (Intersection(3, 7, 8), vec![5, 14]),

    // Row 2
    (Intersection(4, 9, 10), vec![8, 17]),
    (Intersection(4, 5, 10), vec![0, 7, 9]),
    (Intersection(5, 10, 11), vec![8, 10, 19]),
    (Intersection(5, 6, 11), vec![2, 9, 11]),
    (Intersection(6, 11, 12), vec![10, 12, 21]),
    (Intersection(6, 7, 12), vec![4, 11, 13]),
    (Intersection(7, 12, 13), vec![12, 14, 23]),
    (Intersection(7, 8, 13), vec![6, 13, 15]),
    (Intersection(8, 13, 14), vec![14, 25]),

    // Row 3
    (Intersection(9, 15, 16), vec![17, 27]),
    (Intersection(9, 10, 16), vec![7, 16, 18]),
    (Intersection(10, 16, 17), vec![17, 19, 29]),
    (Intersection(10, 11, 17), vec![9, 18, 20]),
    (Intersection(11, 17, 18), vec![19, 21, 31]),
    (Intersection(11, 12, 18), vec![11, 20, 22]),
    (Intersection(12, 18, 19), vec![21, 23, 33]),
    (Intersection(12, 13, 19), vec![13, 22, 24]),
    (Intersection(13, 19, 20), vec![23, 25, 35]),
    (Intersection(13, 14, 20), vec![15, 24, 26]),
    (Intersection(14, 20, 21), vec![25, 37]),

    // Row 4
    (Intersection(15, 16, 22), vec![16, 28]),
    (Intersection(16, 22, 23), vec![27, 29, 38]),
    (Intersection(16, 17, 23), vec![18, 28, 30]),
    (Intersection(17, 23, 24), vec![29, 31, 40]),
    (Intersection(17, 18, 24), vec![20, 30, 32]),
    (Intersection(18, 24, 25), vec![31, 33, 42]),
    (Intersection(18, 19, 25), vec![22, 32, 34]),
    (Intersection(19, 25, 26), vec![33, 35, 44]),
    (Intersection(19, 20, 26), vec![24, 34, 36]),
    (Intersection(20, 26, 27), vec![35, 37, 46]),
    (Intersection(20, 21, 27), vec![26, 36]),

    // Row 5
    (Intersection(22, 23, 28), vec![28, 39]),
    (Intersection(23, 28, 29), vec![38, 40, 47]),
    (Intersection(23, 24, 29), vec![30, 39, 41]),
    (Intersection(24, 29, 30), vec![40, 42, 49]),
    (Intersection(24, 25, 30), vec![32, 41, 43]),
    (Intersection(25, 30, 31), vec![42, 44, 51]),
    (Intersection(25, 26, 31), vec![34, 43, 45]),
    (Intersection(26, 31, 32), vec![44, 46, 53]),
    (Intersection(26, 27, 32), vec![36, 45]),

    // Row 6
    (Intersection(28, 29, 33), vec![39, 48]),
    (Intersection(29, 33, 34), vec![47, 49]),
    (Intersection(29, 30, 34), vec![41, 48, 50]),
    (Intersection(30, 34, 35), vec![49, 51]),
    (Intersection(30, 31, 35), vec![43, 50, 52]),
    (Intersection(31, 35, 36), vec![51, 53]),
    (Intersection(31, 32, 36), vec![45, 52]),
]);
}

#[derive(Debug, PartialEq)]
pub enum BuildingTypes {
    Empty,
    City,
    Settlement,
    Road,
}

const INVALID_PLAYER: u8 = 255;

#[derive(Debug)]
pub struct Building {
    pub player_id: u8,
    pub building_type: BuildingTypes,
}

impl Building {
    pub fn new() -> Building {
        Building {
            player_id: INVALID_PLAYER,
            building_type: BuildingTypes::Empty,
        }
    }

    pub fn create(player_id: u8, building_type: BuildingTypes) -> Building {
        Building {
            player_id: player_id,
            building_type: building_type,
        }
    }
}
