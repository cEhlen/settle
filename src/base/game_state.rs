use std::collections::HashMap;

use crate::base::board::Building;
use crate::base::board::BuildingTypes;
use crate::base::board::Intersection;
use crate::base::board::ALL_INTERSECTIONS;
use crate::base::board::INTERSECTION_CONNECTIONS;

#[derive(Debug)]
enum TileTypes {
    Water,
    Desert,
    Brick,
    Ore,
    Wheat,
    Sheep,
    Wood,
}

#[derive(Debug)]
enum DevelopmentCardType {
    Monopoly,
    Knight,
    RoadBuilding,
    YearOfPlenty,
    VictoryPoint,
}

#[derive(Debug)]
pub struct Tile {
    number: u8,
    tile_type: TileTypes,
}

#[derive(Debug)]
struct HandDevelopmentCard {
    aquired_at_turn: u32,
    card_type: DevelopmentCardType,
}

#[derive(Debug)]
pub struct Bank {
    development_cards: Vec<DevelopmentCardType>,
    resources: Resources,
}

#[derive(Debug)]
struct Resources {
    wood: u8,
    brick: u8,
    sheep: u8,
    wheat: u8,
    ore: u8,
}

#[derive(Debug)]
pub struct Player {
    id: u8,

    resources: Resources,

    development_cards: Vec<HandDevelopmentCard>,

    roads: u8,
    settlements: u8,
    cities: u8,

    victory_points: u8,
}

#[derive(Debug)]
pub enum Phase {
    Startup,
    StartupReverse,
    WaitingForRoll,
    MoveingRobber,
    Discarding,
    Game,
    End,
}

#[derive(Debug)]
pub struct GameState {
    pub phase: Phase,

    pub active_player: u8,

    pub board: Vec<Tile>,
    pub bank: Bank,
    pub player: Vec<Player>,

    pub buildings: HashMap<Intersection, Building>,
}

impl Tile {
    fn new(number: u8, tile_type: TileTypes) -> Tile {
        Tile {
            number: number,
            tile_type: tile_type,
        }
    }

    fn water() -> Tile {
        Tile {
            number: 0,
            tile_type: TileTypes::Water,
        }
    }

    fn desert() -> Tile {
        Tile {
            number: 0,
            tile_type: TileTypes::Desert,
        }
    }

    fn wood(number: u8) -> Tile {
        Tile {
            number: number,
            tile_type: TileTypes::Wood,
        }
    }

    fn brick(number: u8) -> Tile {
        Tile {
            number: number,
            tile_type: TileTypes::Brick,
        }
    }

    fn sheep(number: u8) -> Tile {
        Tile {
            number: number,
            tile_type: TileTypes::Sheep,
        }
    }

    fn wheat(number: u8) -> Tile {
        Tile {
            number: number,
            tile_type: TileTypes::Wheat,
        }
    }

    fn ore(number: u8) -> Tile {
        Tile {
            number: number,
            tile_type: TileTypes::Ore,
        }
    }
}

impl Player {
    fn new(index: u8) -> Player {
        let resources = Resources {
            wood: 0,
            brick: 0,
            sheep: 0,
            wheat: 0,
            ore: 0,
        };

        Player {
            id: index,

            resources: resources,

            development_cards: Vec::new(),

            roads: 15,
            settlements: 5,
            cities: 4,

            victory_points: 0,
        }
    }
}

impl Bank {
    fn new() -> Bank {
        let dev_cards = vec![
            // 5 VPs
            DevelopmentCardType::VictoryPoint,
            DevelopmentCardType::VictoryPoint,
            DevelopmentCardType::VictoryPoint,
            DevelopmentCardType::VictoryPoint,
            DevelopmentCardType::VictoryPoint,
            // 2 Mono
            DevelopmentCardType::Monopoly,
            DevelopmentCardType::Monopoly,
            // 2 Road Building
            DevelopmentCardType::RoadBuilding,
            DevelopmentCardType::RoadBuilding,
            // 2 YoP
            DevelopmentCardType::YearOfPlenty,
            DevelopmentCardType::YearOfPlenty,
            // 14 Knights
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
            DevelopmentCardType::Knight,
        ];

        // TODO: Shuffle Dev Cards

        let resources = Resources {
            wood: 19,
            brick: 19,
            sheep: 19,
            wheat: 19,
            ore: 19,
        };

        Bank {
            resources: resources,

            development_cards: dev_cards,
        }
    }
}

impl GameState {
    pub fn new(number_players: u8) -> GameState {
        let mut player: Vec<Player> = Vec::new();

        for i in 0..number_players {
            player.push(Player::new(i))
        }

        let mut buildings = HashMap::new();
        for intersection in ALL_INTERSECTIONS {
            buildings.insert(intersection, Building::new());
        }

        let mut state = GameState {
            phase: Phase::Startup,

            active_player: 0,
            player,
            buildings,

            bank: Bank::new(),
            board: Vec::new(),
        };

        state.fill_default_board();
        state
    }

    fn fill_default_board(&mut self) {
        self.board = vec![
            Tile::water(),
            Tile::water(),
            Tile::water(),
            Tile::water(),
            Tile::water(),
            Tile::wood(11),
            Tile::sheep(12),
            Tile::wheat(9),
            Tile::water(),
            Tile::water(),
            Tile::brick(4),
            Tile::ore(6),
            Tile::brick(5),
            Tile::sheep(10),
            Tile::water(),
            Tile::water(),
            Tile::desert(),
            Tile::wood(3),
            Tile::wheat(11),
            Tile::wood(4),
            Tile::wheat(8),
            Tile::water(),
            Tile::water(),
            Tile::brick(8),
            Tile::sheep(10),
            Tile::sheep(9),
            Tile::ore(3),
            Tile::water(),
            Tile::water(),
            Tile::ore(5),
            Tile::wheat(2),
            Tile::wood(6),
            Tile::water(),
            Tile::water(),
            Tile::water(),
            Tile::water(),
            Tile::water(),
        ]
    }

    pub fn count_buildings_for_player(&self, player_id: u8, building_type: BuildingTypes) -> u32 {
        self.buildings
            .iter()
            .filter(|(_, building)| {
                building.building_type == building_type && building.player_id == player_id
            })
            .count()
            .try_into()
            .unwrap()
    }

    pub fn get_building_at_intersection(&self, intersection: Intersection) -> Option<&Building> {
        self.buildings.get(&intersection)
    }

    pub fn get_buildings_on_intersections_near(&self, intersection: &Intersection) -> Vec<&Building> {
        match INTERSECTION_CONNECTIONS.get(&intersection) {
            None => vec![],
            Some(neighbours) => neighbours
                .into_iter()
                .map(|intersection| {
                    let inter = &ALL_INTERSECTIONS[*intersection as usize];
                    self.buildings.get(inter)
                })
                .filter(|building| building.is_some())
                .map(|building| building.unwrap())
                .collect(),
        }
    }

    pub fn add_building(&mut self, player_id: u8, building_type: BuildingTypes, intersection: Intersection) {
        *self.buildings.entry(intersection).or_insert(Building::new()) = Building::create(player_id, building_type);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_count_buildings_for_player() {
        let mut state = GameState::new(4);
        state.add_building(1, BuildingTypes::Settlement, Intersection::new(0, 1, 5));
        state.add_building(1, BuildingTypes::Settlement, Intersection::new(9, 15, 16));
        assert_eq!(state.count_buildings_for_player(1, BuildingTypes::Settlement), 2);
        assert_eq!(state.count_buildings_for_player(1, BuildingTypes::City), 0);
        assert_eq!(state.count_buildings_for_player(2, BuildingTypes::Settlement), 0);

    }

    #[test]
    fn test_add_building() {
        let mut state = GameState::new(4);
        state.add_building(1, BuildingTypes::Settlement, Intersection::new(0, 1, 5));
        let expected_building = Building::create(1, BuildingTypes::Settlement);
        if let Some(building) = state.get_building_at_intersection(Intersection::new(0, 1, 5)) {
            assert_eq!(*building, expected_building)
        } else {
            assert!(false)       
        }
    }
}
