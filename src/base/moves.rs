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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_place_settlement_ok_startup() {
        let mut state = GameState::new(4);
        let result = apply_move(0, state, Move::PlaceSettlement(Intersection::new(0, 1, 5)));
        assert!(result.is_ok());
        if let Ok(new_state) = result {
            assert!(new_state.get_building_at_intersection(Intersection::new(0, 1, 5)).is_some())
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_place_settlement_fail_wrong_player_startup() {
        let mut state = GameState::new(4);
        let result = apply_move(2, state, Move::PlaceSettlement(Intersection::new(0, 1, 5)));
        assert!(result.is_err());
    }

    #[test]
    fn test_place_settlement_fail_player_placing_twice_startup() {
        let mut state = GameState::new(4);
        let result = apply_move(0, state, Move::PlaceSettlement(Intersection::new(0, 1, 5)));
        assert!(result.is_ok());
        if let Ok(new_state) = result {
            let r2 = apply_move(0, new_state, Move::PlaceSettlement(Intersection::new(22, 23, 28)));
            assert!(r2.is_err())
        } else {
            assert!(false)
        }
    }
}
