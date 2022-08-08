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
struct Tile {
    number: u8,
    tile_type: TileTypes,
}

#[derive(Debug)]
struct HandDevelopmentCard {
    aquired_at_turn: u32,
    card_type: DevelopmentCardType,
}

#[derive(Debug)]
struct Bank {
    development_cards: Vec<DevelopmentCardType>,
    wood: u8,
    brick: u8,
    sheep: u8,
    wheat: u8,
    ore: u8,
}

#[derive(Debug)]
struct Player {
    id: u8,

    wood: u8,
    brick: u8,
    sheep: u8,
    wheat: u8,
    ore: u8,
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
        Player {
            id: index,

            wood: 0,
            brick: 0,
            sheep: 0,
            wheat: 0,
            ore: 0,

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

        Bank {
            wood: 19,
            brick: 19,
            sheep: 19,
            wheat: 19,
            ore: 19,

            development_cards: dev_cards,
        }
    }
}

impl GameState {
    fn new(number_players: u8) -> GameState {
        let mut player: Vec<Player> = Vec::new();

        for i in 0..number_players {
            player.push(Player::new(i))
        }

        let mut state = GameState {
            phase: Phase::Startup,

            active_player: 0,
            player: player,

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
}
