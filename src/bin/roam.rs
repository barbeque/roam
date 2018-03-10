extern crate roam;
extern crate pancurses;
use pancurses::{initscr, endwin, Input};
use roam::map::{Dungeon, generate_map};
use roam::entity::{Entity};

struct GameState {
    offset_x: i32,
    offset_y: i32,
    player: Entity,
    dungeon: Dungeon
}
impl GameState {
    fn new(dungeon: Dungeon) -> GameState {
        GameState {
            offset_x: 0,
            offset_y: 0,
            player: Entity::new(),
            dungeon: dungeon
        }
    }
}

fn raster_entity(window: &pancurses::Window, state: &GameState, entity: &Entity) {
    // figure out where on screen the real position is
    let screen_x = entity.location_x - state.offset_x;
    let screen_y = entity.location_y - state.offset_y;
    window.mvprintw(screen_y, screen_x, &entity.view.to_string());
}

fn raster_screen(window: &pancurses::Window, state: &GameState) {
    window.erase();

    let max_x = window.get_max_x();
    let max_y = window.get_max_y();
    let dungeon = &state.dungeon;

    // Draw dungeon map
    for y in 0..(max_y - 1) {
        let tile_y = state.offset_y + y;

        if tile_y < 0 || tile_y as usize >= dungeon.get_height() {
            continue;
        }

        for x in 0..(max_x) {
            let tile_x = state.offset_x + x;

            if tile_x < 0 || tile_x as usize >= dungeon.get_width() {
                continue;
            }

            let actual_tile = dungeon.get_at(tile_x as usize, tile_y as usize);
            window.mvprintw(y, x, &actual_tile.to_string()); // do i really need a to_string
        }
    }

    // Display entities - just player for now
    raster_entity(&window, &state, &state.player);

    // Draw window UI
    for x in 0..(max_x) {
        window.mvprintw(max_y - 1, x, "*");
    }

    window.mvprintw(max_y - 1, 3,
        &format!("HP: {}/{}", state.player.hit_points, state.player.max_hit_points));
}

fn move_player(state: &mut GameState, dx: i32, dy: i32) -> bool {
    // TODO: Scroll offset_x, offset_y as the player 'moves off screen'
    let proposed_x = state.player.location_x + dx; // TODO: collision ray in case dx > 1
    let proposed_y = state.player.location_y + dy; // TODO: collision ray in case dy > 1

    if proposed_x >= 0 && proposed_y >= 0
        && proposed_x < state.dungeon.get_width() as i32 && proposed_y < state.dungeon.get_height() as i32 {
        if state.dungeon.get_at(proposed_x as usize, proposed_y as usize) != '#' { // hack for now
            state.player.location_x = proposed_x;
            state.player.location_y = proposed_y;
            return true;
        }
    }

    false
}

fn main() {
    let window = initscr();
    window.keypad(true);
    pancurses::curs_set(0); // hide insertion pointer
    pancurses::noecho();

    let dungeon = generate_map();
    let mut game_state = GameState::new(dungeon);

    // HACK: For now, find the location of the closest room and put the player in it
    let (x, y) = game_state.dungeon.find_room_tile();
    game_state.player.location_x = x;
    game_state.player.location_y = y;

    // Continue
    loop {
        raster_screen(&window, &game_state);
        window.refresh();

        match window.getch() {
            Some(Input::Character(c)) => {
                match c {
                    'h' => { move_player(&mut game_state, -1, 0); },
                    'j' => { move_player(&mut game_state, 0, 1); },
                    'k' => { move_player(&mut game_state, 0, -1); },
                    'l' => { move_player(&mut game_state, 1, 0); },
                    _ => break
                }
            },
            Some(Input::KeyLeft) => { move_player(&mut game_state, -1, 0); },
            Some(Input::KeyUp) => { move_player(&mut game_state, 0, -1); },
            Some(Input::KeyDown) => { move_player(&mut game_state, 0, 1); },
            Some(Input::KeyRight) => { move_player(&mut game_state, 1, 0); },
            Some(Input::KeyDC) => break,
            Some(Input::KeyResize) => {
                pancurses::resize_term(0, 0);
            }
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            },
            None => ()
        }
    }

    endwin();
}
