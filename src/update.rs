use map::Dungeon;
use entity::Entity;

// Game update functionality
pub fn move_player(player: &mut Entity, dungeon: &Dungeon, dx: i32, dy: i32) -> bool {
    // TODO: Scroll offset_x, offset_y as the player 'moves off screen'
    let proposed_x = player.location_x + dx; // TODO: collision ray in case dx > 1
    let proposed_y = player.location_y + dy; // TODO: collision ray in case dy > 1

    if proposed_x >= 0 && proposed_y >= 0
        && proposed_x < dungeon.get_width() as i32 && proposed_y < dungeon.get_height() as i32 {
        if dungeon.get_at(proposed_x as usize, proposed_y as usize) != '#' { // hack for now
            player.location_x = proposed_x;
            player.location_y = proposed_y;
            return true;
        }
    }

    false
}
