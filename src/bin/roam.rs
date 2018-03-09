extern crate roam;
extern crate pancurses;
use pancurses::{initscr, endwin, Input};
use roam::map::{Dungeon, generate_map};
use roam::entity::{Entity};

struct GameState {
    offset_x: i32,
    offset_y: i32,
    player: Entity
}
impl GameState {
    fn new() -> GameState {
        GameState {
            offset_x: 0,
            offset_y: 0,
            player: Entity::new()
        }
    }
}

fn raster_entity(window: &pancurses::Window, dungeon: &Dungeon, state: &GameState, entity: &Entity) {
    // figure out where on screen the real position is
    let screen_x = entity.location_x - state.offset_x;
    let screen_y = entity.location_y - state.offset_y;
    window.mvprintw(screen_y, screen_x, &entity.view.to_string());
}

fn raster_screen(window: &pancurses::Window, dungeon: &Dungeon, state: &GameState) {
    window.erase();

    let max_x = window.get_max_x();
    let max_y = window.get_max_y();

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
    raster_entity(&window, &dungeon, &state, &state.player);

    // Draw window UI
    for x in 0..(max_x) {
        window.mvprintw(max_y - 1, x, "*");
    }

    window.mvprintw(max_y - 1, 3,
        &format!("HP: {}/{}", state.player.hit_points, state.player.max_hit_points));
}

fn main() {
    let window = initscr();
    window.keypad(true);
    pancurses::curs_set(0); // hide insertion pointer
    pancurses::noecho();

    let dungeon = generate_map();
    let mut game_state = GameState::new();

    // HACK: For now, find the location of the closest room and put the player in it
    let (x, y) = dungeon.find_room_tile();
    game_state.player.location_x = x;
    game_state.player.location_y = y;

    // Continue
    const SCROLL_SPEED : i32 = 3;

    loop {
        raster_screen(&window, &dungeon, &game_state);
        window.refresh();

        match window.getch() {
            Some(Input::Character(c)) => {
                match c {
                    'h' => game_state.offset_x -= SCROLL_SPEED,
                    'j' => game_state.offset_y += SCROLL_SPEED,
                    'k' => game_state.offset_y -= SCROLL_SPEED,
                    'l' => game_state.offset_x += SCROLL_SPEED,
                    _ => break
                }
            },
            Some(Input::KeyLeft) => game_state.offset_x -= SCROLL_SPEED,
            Some(Input::KeyUp) => game_state.offset_y -= SCROLL_SPEED,
            Some(Input::KeyDown) => game_state.offset_y += SCROLL_SPEED,
            Some(Input::KeyRight) => game_state.offset_x += SCROLL_SPEED,
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
