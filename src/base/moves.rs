use crate::base::game_state::GameState;
use crate::base::game_state::Phase;
use crate::base::board::Intersection;
use crate::base::board::RoadPosition;

enum Move {
    PlaceSettlement(Intersection),
    PlaceRoad(RoadPosition),
}

#[derive(Debug)]
struct InvalidMoveError;

type ApplyMoveResult = std::result::Result<GameState, InvalidMoveError>;

pub fn apply_move(player_id: u8, game_state: GameState, game_move: Move) -> ApplyMoveResult {
    match game_move {
        Move::PlaceSettlement(intersection) => handle_place_settlement(player_id, game_state, intersection),
        _ => Err(InvalidMoveError),
    }
}

fn handle_place_settlement(player_id: u8, game_state: GameState, intersection: Intersection) -> ApplyMoveResult {
    if player_id != game_state.active_player {
        return Err(InvalidMoveError)
    }
    match game_state.phase {
        Phase::Startup => Ok(game_state),
        _ => Err(InvalidMoveError),
    }
}
