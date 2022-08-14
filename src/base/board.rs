use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Hash, Copy, Clone)]
pub struct Intersection {
    p1: u8,
    p2: u8,
    p3: u8,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2 && self.p3 == other.p3
    }
}

impl Eq for Intersection {}

impl Intersection {
    pub fn new(p1: u8, p2: u8, p3: u8) -> Intersection {
        Intersection { p1, p2, p3 }
    }
}

#[derive(Debug, Hash)]
pub struct RoadPosition {
    p1: u8,
    p2: u8,
}

impl PartialEq for RoadPosition {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2
    }
}

impl Eq for RoadPosition {}

impl RoadPosition {
    pub fn new(p1: u8, p2: u8) -> RoadPosition {
        RoadPosition { p1, p2 }
    }
}

pub const ALL_INTERSECTIONS: [Intersection; 54] = [
    // Row 1
    Intersection {
        p1: 0,
        p2: 4,
        p3: 5,
    },
    Intersection {
        p1: 0,
        p2: 1,
        p3: 5,
    },
    Intersection {
        p1: 1,
        p2: 5,
        p3: 6,
    },
    Intersection {
        p1: 1,
        p2: 2,
        p3: 6,
    },
    Intersection {
        p1: 2,
        p2: 6,
        p3: 7,
    },
    Intersection {
        p1: 2,
        p2: 3,
        p3: 7,
    },
    Intersection {
        p1: 3,
        p2: 7,
        p3: 8,
    },
    // Row 2
    Intersection {
        p1: 4,
        p2: 9,
        p3: 10,
    },
    Intersection {
        p1: 4,
        p2: 5,
        p3: 10,
    },
    Intersection {
        p1: 5,
        p2: 10,
        p3: 11,
    },
    Intersection {
        p1: 5,
        p2: 6,
        p3: 11,
    },
    Intersection {
        p1: 6,
        p2: 11,
        p3: 12,
    },
    Intersection {
        p1: 6,
        p2: 7,
        p3: 12,
    },
    Intersection {
        p1: 7,
        p2: 12,
        p3: 13,
    },
    Intersection {
        p1: 7,
        p2: 8,
        p3: 13,
    },
    Intersection {
        p1: 8,
        p2: 13,
        p3: 14,
    },
    // Row 3
    Intersection {
        p1: 9,
        p2: 15,
        p3: 16,
    },
    Intersection {
        p1: 9,
        p2: 10,
        p3: 16,
    },
    Intersection {
        p1: 10,
        p2: 16,
        p3: 17,
    },
    Intersection {
        p1: 10,
        p2: 11,
        p3: 17,
    },
    Intersection {
        p1: 11,
        p2: 17,
        p3: 18,
    },
    Intersection {
        p1: 11,
        p2: 12,
        p3: 18,
    },
    Intersection {
        p1: 12,
        p2: 18,
        p3: 19,
    },
    Intersection {
        p1: 12,
        p2: 13,
        p3: 19,
    },
    Intersection {
        p1: 13,
        p2: 19,
        p3: 20,
    },
    Intersection {
        p1: 13,
        p2: 14,
        p3: 20,
    },
    Intersection {
        p1: 14,
        p2: 20,
        p3: 21,
    },
    // Row 4
    Intersection {
        p1: 15,
        p2: 16,
        p3: 22,
    },
    Intersection {
        p1: 16,
        p2: 22,
        p3: 23,
    },
    Intersection {
        p1: 16,
        p2: 17,
        p3: 23,
    },
    Intersection {
        p1: 17,
        p2: 23,
        p3: 24,
    },
    Intersection {
        p1: 17,
        p2: 18,
        p3: 24,
    },
    Intersection {
        p1: 18,
        p2: 24,
        p3: 25,
    },
    Intersection {
        p1: 18,
        p2: 19,
        p3: 25,
    },
    Intersection {
        p1: 19,
        p2: 25,
        p3: 26,
    },
    Intersection {
        p1: 19,
        p2: 20,
        p3: 26,
    },
    Intersection {
        p1: 20,
        p2: 26,
        p3: 27,
    },
    Intersection {
        p1: 20,
        p2: 21,
        p3: 27,
    },
    // Row 5
    Intersection {
        p1: 22,
        p2: 23,
        p3: 28,
    },
    Intersection {
        p1: 23,
        p2: 28,
        p3: 29,
    },
    Intersection {
        p1: 23,
        p2: 24,
        p3: 29,
    },
    Intersection {
        p1: 24,
        p2: 29,
        p3: 30,
    },
    Intersection {
        p1: 24,
        p2: 25,
        p3: 30,
    },
    Intersection {
        p1: 25,
        p2: 30,
        p3: 31,
    },
    Intersection {
        p1: 25,
        p2: 26,
        p3: 31,
    },
    Intersection {
        p1: 26,
        p2: 31,
        p3: 32,
    },
    Intersection {
        p1: 26,
        p2: 27,
        p3: 32,
    },
    // Row 6
    Intersection {
        p1: 28,
        p2: 29,
        p3: 33,
    },
    Intersection {
        p1: 29,
        p2: 33,
        p3: 34,
    },
    Intersection {
        p1: 29,
        p2: 30,
        p3: 34,
    },
    Intersection {
        p1: 30,
        p2: 34,
        p3: 35,
    },
    Intersection {
        p1: 30,
        p2: 31,
        p3: 35,
    },
    Intersection {
        p1: 31,
        p2: 35,
        p3: 36,
    },
    Intersection {
        p1: 31,
        p2: 32,
        p3: 36,
    },
];

pub const ALL_ROAD_POSITIONS: [RoadPosition; 72] = [
    // Row 1
    RoadPosition { p1: 0, p2: 5 }, // 0
    RoadPosition { p1: 1, p2: 5 },
    RoadPosition { p1: 1, p2: 6 },
    RoadPosition { p1: 2, p2: 6 },
    RoadPosition { p1: 2, p2: 7 },
    RoadPosition { p1: 3, p2: 7 }, // 5
    RoadPosition { p1: 4, p2: 5 },
    RoadPosition { p1: 5, p2: 6 },
    RoadPosition { p1: 6, p2: 7 },
    RoadPosition { p1: 7, p2: 8 }, // 9
    // Row 2
    RoadPosition { p1: 4, p2: 10 }, // 10
    RoadPosition { p1: 5, p2: 10 },
    RoadPosition { p1: 5, p2: 11 },
    RoadPosition { p1: 6, p2: 11 },
    RoadPosition { p1: 6, p2: 12 },
    RoadPosition { p1: 7, p2: 12 },
    RoadPosition { p1: 7, p2: 13 },
    RoadPosition { p1: 8, p2: 13 },
    RoadPosition { p1: 9, p2: 10 },
    RoadPosition { p1: 10, p2: 11 },
    RoadPosition { p1: 11, p2: 12 },
    RoadPosition { p1: 12, p2: 13 },
    RoadPosition { p1: 13, p2: 14 },
    // Row 3
    RoadPosition { p1: 9, p2: 16 },
    RoadPosition { p1: 10, p2: 16 },
    RoadPosition { p1: 10, p2: 17 },
    RoadPosition { p1: 11, p2: 17 },
    RoadPosition { p1: 11, p2: 18 },
    RoadPosition { p1: 12, p2: 18 },
    RoadPosition { p1: 12, p2: 19 },
    RoadPosition { p1: 13, p2: 19 },
    RoadPosition { p1: 13, p2: 20 },
    RoadPosition { p1: 14, p2: 20 },
    RoadPosition { p1: 15, p2: 16 },
    RoadPosition { p1: 16, p2: 17 },
    RoadPosition { p1: 17, p2: 18 },
    RoadPosition { p1: 18, p2: 19 },
    RoadPosition { p1: 19, p2: 20 },
    RoadPosition { p1: 20, p2: 21 },
    // Row 4
    RoadPosition { p1: 16, p2: 22 },
    RoadPosition { p1: 16, p2: 23 },
    RoadPosition { p1: 17, p2: 23 },
    RoadPosition { p1: 17, p2: 24 },
    RoadPosition { p1: 18, p2: 24 },
    RoadPosition { p1: 18, p2: 25 },
    RoadPosition { p1: 19, p2: 25 },
    RoadPosition { p1: 19, p2: 26 },
    RoadPosition { p1: 20, p2: 26 },
    RoadPosition { p1: 20, p2: 27 },
    RoadPosition { p1: 22, p2: 23 },
    RoadPosition { p1: 23, p2: 24 },
    RoadPosition { p1: 24, p2: 25 },
    RoadPosition { p1: 25, p2: 26 },
    RoadPosition { p1: 26, p2: 27 },
    // Row 5
    RoadPosition { p1: 23, p2: 28 },
    RoadPosition { p1: 23, p2: 29 },
    RoadPosition { p1: 24, p2: 29 },
    RoadPosition { p1: 24, p2: 30 },
    RoadPosition { p1: 25, p2: 30 },
    RoadPosition { p1: 25, p2: 31 },
    RoadPosition { p1: 26, p2: 31 },
    RoadPosition { p1: 26, p2: 32 },
    RoadPosition { p1: 28, p2: 29 },
    RoadPosition { p1: 29, p2: 30 },
    RoadPosition { p1: 30, p2: 31 },
    RoadPosition { p1: 31, p2: 32 },
    // Row 6
    RoadPosition { p1: 29, p2: 33 },
    RoadPosition { p1: 29, p2: 34 },
    RoadPosition { p1: 30, p2: 34 },
    RoadPosition { p1: 30, p2: 35 },
    RoadPosition { p1: 31, p2: 35 },
    RoadPosition { p1: 31, p2: 36 },
];

lazy_static! {
pub static ref INTERSECTION_CONNECTIONS: HashMap<Intersection, Vec<u8>> = HashMap::from([
    // Row 1
    (Intersection::new(0, 4, 5), vec![1, 8]),
    (Intersection::new(0, 1, 5), vec![0, 2]),
    (Intersection::new(1, 5, 6), vec![1, 3, 10]),
    (Intersection::new(1, 2, 6), vec![2, 4]),
    (Intersection::new(2, 6, 7), vec![3, 5, 12]),
    (Intersection::new(2, 3, 7), vec![4, 6]),
    (Intersection::new(3, 7, 8), vec![5, 14]),

    // Row 2
    (Intersection::new(4, 9, 10), vec![8, 17]),
    (Intersection::new(4, 5, 10), vec![0, 7, 9]),
    (Intersection::new(5, 10, 11), vec![8, 10, 19]),
    (Intersection::new(5, 6, 11), vec![2, 9, 11]),
    (Intersection::new(6, 11, 12), vec![10, 12, 21]),
    (Intersection::new(6, 7, 12), vec![4, 11, 13]),
    (Intersection::new(7, 12, 13), vec![12, 14, 23]),
    (Intersection::new(7, 8, 13), vec![6, 13, 15]),
    (Intersection::new(8, 13, 14), vec![14, 25]),

    // Row 3
    (Intersection::new(9, 15, 16), vec![17, 27]),
    (Intersection::new(9, 10, 16), vec![7, 16, 18]),
    (Intersection::new(10, 16, 17), vec![17, 19, 29]),
    (Intersection::new(10, 11, 17), vec![9, 18, 20]),
    (Intersection::new(11, 17, 18), vec![19, 21, 31]),
    (Intersection::new(11, 12, 18), vec![11, 20, 22]),
    (Intersection::new(12, 18, 19), vec![21, 23, 33]),
    (Intersection::new(12, 13, 19), vec![13, 22, 24]),
    (Intersection::new(13, 19, 20), vec![23, 25, 35]),
    (Intersection::new(13, 14, 20), vec![15, 24, 26]),
    (Intersection::new(14, 20, 21), vec![25, 37]),

    // Row 4
    (Intersection::new(15, 16, 22), vec![16, 28]),
    (Intersection::new(16, 22, 23), vec![27, 29, 38]),
    (Intersection::new(16, 17, 23), vec![18, 28, 30]),
    (Intersection::new(17, 23, 24), vec![29, 31, 40]),
    (Intersection::new(17, 18, 24), vec![20, 30, 32]),
    (Intersection::new(18, 24, 25), vec![31, 33, 42]),
    (Intersection::new(18, 19, 25), vec![22, 32, 34]),
    (Intersection::new(19, 25, 26), vec![33, 35, 44]),
    (Intersection::new(19, 20, 26), vec![24, 34, 36]),
    (Intersection::new(20, 26, 27), vec![35, 37, 46]),
    (Intersection::new(20, 21, 27), vec![26, 36]),

    // Row 5
    (Intersection::new(22, 23, 28), vec![28, 39]),
    (Intersection::new(23, 28, 29), vec![38, 40, 47]),
    (Intersection::new(23, 24, 29), vec![30, 39, 41]),
    (Intersection::new(24, 29, 30), vec![40, 42, 49]),
    (Intersection::new(24, 25, 30), vec![32, 41, 43]),
    (Intersection::new(25, 30, 31), vec![42, 44, 51]),
    (Intersection::new(25, 26, 31), vec![34, 43, 45]),
    (Intersection::new(26, 31, 32), vec![44, 46, 53]),
    (Intersection::new(26, 27, 32), vec![36, 45]),

    // Row 6
    (Intersection::new(28, 29, 33), vec![39, 48]),
    (Intersection::new(29, 33, 34), vec![47, 49]),
    (Intersection::new(29, 30, 34), vec![41, 48, 50]),
    (Intersection::new(30, 34, 35), vec![49, 51]),
    (Intersection::new(30, 31, 35), vec![43, 50, 52]),
    (Intersection::new(31, 35, 36), vec![51, 53]),
    (Intersection::new(31, 32, 36), vec![45, 52]),
]);

pub static ref ROAD_TO_INTERSECTION_CONNECTION: HashMap<RoadPosition, [u8; 2]> = HashMap::from([
    // Row 1
    (RoadPosition{ p1: 0, p2: 5 }, [0, 1]),
    (RoadPosition{ p1: 1, p2: 5 }, [1, 2]),
    (RoadPosition{ p1: 1, p2: 6}, [2, 3]),
    (RoadPosition{ p1: 2, p2: 6}, [3, 4]),
    (RoadPosition{p1: 2, p2: 7}, [4, 5]),
    (RoadPosition{ p1: 3, p2: 7}, [5, 6]),
    (RoadPosition{ p1: 4, p2: 5}, [0, 8]),
    (RoadPosition{ p1: 5, p2: 6}, [2, 10]),
    (RoadPosition{ p1: 6, p2: 7}, [4, 12]),
    (RoadPosition{ p1: 7, p2: 8}, [6, 14]),
    // Row 2
    (RoadPosition{ p1: 4, p2: 10}, [7, 8]),
    (RoadPosition{ p1: 5, p2: 10}, [8, 9]),
    (RoadPosition{ p1: 5, p2: 11}, [9, 10]),
    (RoadPosition{ p1: 6, p2: 11}, [10, 11]),
    (RoadPosition{ p1: 6, p2: 12}, [11, 12]),
    (RoadPosition{ p1: 7, p2: 12}, [12, 13]),
    (RoadPosition{ p1: 7, p2: 13}, [13, 14]),
    (RoadPosition{ p1: 8, p2: 13}, [14, 15]),
    (RoadPosition{ p1: 9, p2: 10}, [7, 17]),
    (RoadPosition{ p1: 10, p2: 11}, [9, 19]),
    (RoadPosition{ p1: 11, p2: 12}, [11, 21]),
    (RoadPosition{ p1: 12, p2: 13}, [13, 23]),
    (RoadPosition{ p1: 13, p2: 14}, [15, 25]),
    // Row 3
    (RoadPosition{ p1: 9, p2: 16}, [16, 17]),
    (RoadPosition{ p1: 10, p2: 16}, [17, 18]),
    (RoadPosition{ p1: 10, p2: 17}, [18, 19]),
    (RoadPosition{ p1: 11, p2: 17}, [19, 20]),
    (RoadPosition{ p1: 11, p2: 18}, [20, 21]),
    (RoadPosition{ p1: 12, p2: 18}, [21, 22]),
    (RoadPosition{ p1: 12, p2: 19}, [22, 23]),
    (RoadPosition{ p1: 13, p2: 19}, [23, 24]),
    (RoadPosition{ p1: 13, p2: 20}, [24, 25]),
    (RoadPosition{ p1: 14, p2: 20}, [25, 26]),
    (RoadPosition{ p1: 15, p2: 16}, [16, 27]),
    (RoadPosition{ p1: 16, p2: 17}, [18, 29]),
    (RoadPosition{ p1: 17, p2: 18}, [20, 31]),
    (RoadPosition{ p1: 18, p2: 19}, [22, 33]),
    (RoadPosition{ p1: 19, p2: 20}, [24, 35]),
    (RoadPosition{ p1: 20, p2: 21}, [26, 37]),
    // Row 4
    (RoadPosition{ p1: 16, p2: 22}, [27, 28]),
    (RoadPosition{ p1: 16, p2: 23}, [28, 29]),
    (RoadPosition{ p1: 17, p2: 23}, [29, 30]),
    (RoadPosition{ p1: 17, p2: 24}, [30, 31]),
    (RoadPosition{ p1: 18, p2: 24}, [31, 32]),
    (RoadPosition{ p1: 18, p2: 25}, [32, 33]),
    (RoadPosition{ p1: 19, p2: 25}, [33, 34]),
    (RoadPosition{ p1: 19, p2: 26}, [34, 35]),
    (RoadPosition{ p1: 20, p2: 26}, [35, 36]),
    (RoadPosition{ p1: 20, p2: 27}, [36, 37]),
    (RoadPosition{ p1: 22, p2: 23}, [28, 38]),
    (RoadPosition{ p1: 23, p2: 24}, [30, 40]),
    (RoadPosition{ p1: 24, p2: 25}, [32, 42]),
    (RoadPosition{ p1: 25, p2: 26}, [34, 44]),
    (RoadPosition{ p1: 26, p2: 27}, [36, 46]),
    // Row 5
    (RoadPosition{ p1: 23, p2: 28}, [38, 39]),
    (RoadPosition{ p1: 23, p2: 29}, [39, 40]),
    (RoadPosition{ p1: 24, p2: 29}, [40, 41]),
    (RoadPosition{ p1: 24, p2: 30}, [41, 42]),
    (RoadPosition{ p1: 25, p2: 30}, [42, 43]),
    (RoadPosition{ p1: 25, p2: 31}, [43, 44]),
    (RoadPosition{ p1: 26, p2: 31}, [44, 45]),
    (RoadPosition{ p1: 26, p2: 32}, [45, 46]),
    (RoadPosition{ p1: 28, p2: 29}, [39, 47]),
    (RoadPosition{ p1: 29, p2: 30}, [41, 49]),
    (RoadPosition{ p1: 30, p2: 31}, [43, 51]),
    (RoadPosition{ p1: 31, p2: 32}, [45, 53]),
    // Row 6
    (RoadPosition{ p1: 29, p2: 33}, [47, 48]),
    (RoadPosition{ p1: 29, p2: 34}, [48, 49]),
    (RoadPosition{ p1: 30, p2: 34}, [49, 50]),
    (RoadPosition{ p1: 30, p2: 35}, [50, 51]),
    (RoadPosition{ p1: 31, p2: 35}, [51, 52]),
    (RoadPosition{ p1: 31, p2: 36}, [52, 53]),
]);


pub static ref INTERSECTION_TO_ROAD_CONNECTION: HashMap<Intersection, Vec<u8>> = HashMap::from([
    (Intersection::new(13,14,20), vec![22,31,32]),
    (Intersection::new(30,34,35), vec![68,69]),
(Intersection::new(5,6,11), vec![7,12,13]),
(Intersection::new(19,20,26), vec![37,46,47]),
(Intersection::new(29,30,34), vec![63,67,68]),
(Intersection::new(10,11,17), vec![19,25,26]),
(Intersection::new(0,4,5), vec![0,6]),
(Intersection::new(6,11,12), vec![13,14,20]),
(Intersection::new(7,12,13), vec![15,16,21]),
(Intersection::new(26,27,32), vec![53,61]),
(Intersection::new(10,16,17), vec![24,25,34]),
(Intersection::new(1,5,6), vec![1,2,7]),
(Intersection::new(16,22,23), vec![39,40,49]),
(Intersection::new(0,1,5), vec![0,1]),
(Intersection::new(7,8,13), vec![9,16,17]),
(Intersection::new(25,26,31), vec![52,59,60]),
(Intersection::new(26,31,32), vec![60,61,65]),
(Intersection::new(9,10,16), vec![18,23,24]),
(Intersection::new(11,12,18), vec![20,27,28]),
(Intersection::new(3,7,8), vec![5,9]),
(Intersection::new(16,17,23), vec![34,40,41]),
(Intersection::new(12,18,19), vec![28,29,36]),
(Intersection::new(2,6,7), vec![3,4,8]),
(Intersection::new(4,5,10), vec![6,10,11]),
(Intersection::new(11,17,18), vec![26,27,35]),
(Intersection::new(31,35,36), vec![70,71]),
(Intersection::new(18,24,25), vec![43,44,51]),
(Intersection::new(14,20,21), vec![32,38]),
(Intersection::new(4,9,10), vec![10,18]),
(Intersection::new(9,15,16), vec![23,33]),
(Intersection::new(1,2,6), vec![2,3]),
(Intersection::new(5,10,11), vec![11,12,19]),
(Intersection::new(2,3,7), vec![4,5]),
(Intersection::new(30,31,35), vec![64,69,70]),
(Intersection::new(19,25,26), vec![45,46,52]),
(Intersection::new(6,7,12), vec![8,14,15]),
(Intersection::new(20,26,27), vec![47,48,53]),
(Intersection::new(24,29,30), vec![56,57,63]),
(Intersection::new(17,23,24), vec![41,42,50]),
(Intersection::new(23,24,29), vec![50,55,56]),
(Intersection::new(25,30,31), vec![58,59,64]),
(Intersection::new(13,19,20), vec![30,31,37]),
(Intersection::new(8,13,14), vec![17,22]),
(Intersection::new(24,25,30), vec![51,57,58]),
(Intersection::new(18,19,25), vec![36,44,45]),
(Intersection::new(12,13,19), vec![21,29,30]),
(Intersection::new(23,28,29), vec![54,55,62]),
(Intersection::new(28,29,33), vec![62,66]),
(Intersection::new(22,23,28), vec![49,54]),
(Intersection::new(15,16,22), vec![33,39]),
(Intersection::new(17,18,24), vec![35,42,43]),
(Intersection::new(31,32,36), vec![65,71]),
(Intersection::new(20,21,27), vec![38,48]),
(Intersection::new(29,33,34), vec![66,67]),
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

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.player_id == other.player_id && self.building_type == other.building_type
    }
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
