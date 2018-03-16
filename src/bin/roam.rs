extern crate pancurses;
extern crate roam;
use pancurses::{endwin, initscr, Input};
use roam::map::{generate_map, Dungeon};
use roam::entity::Entity;
use roam::update::move_player;

struct GameState {
    // TODO: this probably should be moved into the lib as well
    offset_x: i32,
    offset_y: i32,
    player: Entity,
    dungeon: Dungeon,
}
impl GameState {
    fn new(dungeon: Dungeon) -> GameState {
        GameState {
            offset_x: 0,
            offset_y: 0,
            player: Entity::new(),
            dungeon: dungeon,
        }
    }
}

fn get_screen_loc_for_entity(entity: &Entity, game_state: &GameState) -> (i32, i32) {
    // figure out where on screen the real position is
    let screen_x = entity.location_x - game_state.offset_x;
    let screen_y = entity.location_y - game_state.offset_y;
    (screen_x, screen_y)
}

fn update_scrolling(game_state: &mut GameState, window: &pancurses::Window) {
    // Run just before the game state to make sure the window
    // can still see the player
    
    // Update scroll positions
    let max_x = window.get_max_x();
    let max_y = window.get_max_y();
    const SCROLL_EDGE : i32 = 6;
    const SCROLL_AMOUNT : i32 = 4;

    loop {
        // Repeatedly attempt to scroll the player into view

        let (screen_x, screen_y) = get_screen_loc_for_entity(&game_state.player, &game_state);
        
        if screen_x < SCROLL_EDGE {
            game_state.offset_x -= SCROLL_AMOUNT;
            continue; // we might need more scrolling still
        }
        if screen_y < SCROLL_EDGE {
            game_state.offset_y -= SCROLL_AMOUNT;
            continue;
        }
        if screen_x > max_x - SCROLL_EDGE {
            game_state.offset_x += SCROLL_AMOUNT;
            continue;
        }
        if screen_y > max_y - SCROLL_EDGE {
            game_state.offset_y += SCROLL_AMOUNT;
            continue;
        }

        break; // No more scrolling was needed
    }
}

fn raster_entity(window: &pancurses::Window, state: &GameState, entity: &Entity) {
    let (screen_x, screen_y) = get_screen_loc_for_entity(&entity, &state);
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

    window.mvprintw(
        max_y - 1,
        3,
        &format!(
            "HP: {}/{}",
            state.player.hit_points, state.player.max_hit_points
        ),
    );
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
        update_scrolling(&mut game_state, &window);
        raster_screen(&window, &game_state);
        window.refresh();

        match window.getch() {
            Some(Input::Character(c)) => match c {
                'h' => {
                    move_player(&mut game_state.player, &game_state.dungeon, -1, 0);
                }
                'j' => {
                    move_player(&mut game_state.player, &game_state.dungeon, 0, 1);
                }
                'k' => {
                    move_player(&mut game_state.player, &game_state.dungeon, 0, -1);
                }
                'l' => {
                    move_player(&mut game_state.player, &game_state.dungeon, 1, 0);
                }
                _ => break,
            },
            Some(Input::KeyLeft) => {
                move_player(&mut game_state.player, &game_state.dungeon, -1, 0);
            }
            Some(Input::KeyUp) => {
                move_player(&mut game_state.player, &game_state.dungeon, 0, -1);
            }
            Some(Input::KeyDown) => {
                move_player(&mut game_state.player, &game_state.dungeon, 0, 1);
            }
            Some(Input::KeyRight) => {
                move_player(&mut game_state.player, &game_state.dungeon, 1, 0);
            }
            Some(Input::KeyDC) => break,
            Some(Input::KeyResize) => {
                pancurses::resize_term(0, 0);
            }
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }

    endwin();
}
