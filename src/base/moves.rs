use crate::base::board::Intersection;
use crate::base::board::RoadPosition;
use crate::base::board::BuildingTypes;
use crate::base::game_state::GameState;
use crate::base::game_state::Phase;

pub enum Move {
    PlaceSettlement(Intersection),
    PlaceRoad(RoadPosition),
}

#[derive(Debug)]
struct InvalidMoveError;

pub type ApplyMoveResult = std::result::Result<GameState, InvalidMoveError>;

pub fn apply_move(player_id: u8, mut game_state: GameState, game_move: Move) -> ApplyMoveResult {
    match game_move {
        Move::PlaceSettlement(intersection) => {
            handle_place_settlement(player_id, game_state, intersection)
        }
        _ => Err(InvalidMoveError),
    }
}

fn handle_place_settlement(
    player_id: u8,
    mut game_state: GameState,
    intersection: Intersection,
) -> ApplyMoveResult {
    if player_id != game_state.active_player {
        return Err(InvalidMoveError);
    }
    match game_state.phase {
        Phase::Startup => place_settlement_startup(player_id, game_state, intersection),
        Phase::StartupReverse => Ok(game_state),
        Phase::Game => Ok(game_state),
        _ => Err(InvalidMoveError),
    }
}

fn place_settlement_startup(
    player_id: u8,
    mut game_state: GameState,
    intersection: Intersection,
) -> ApplyMoveResult {
    // First check if the player can actually place a settlement
    // We have to check if she maybe placed a settlement already and needs
    // to place a road instead
    let placed_settlements = game_state.count_buildings_for_player(player_id, BuildingTypes::Settlement);
    if placed_settlements != 0 {
        return Err(InvalidMoveError)
    }
    let nearby_buildings = game_state.get_buildings_on_intersections_near(&intersection);
    if nearby_buildings.len() == 0 {
        return Err(InvalidMoveError)
    }
    game_state.add_building(player_id, BuildingTypes::Settlement, intersection);
    Ok(game_state)
}
