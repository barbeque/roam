extern crate roam;
extern crate pancurses;
use pancurses::{initscr, endwin, Input};
use roam::map::{Dungeon, generate_map};

fn raster_screen(window: &pancurses::Window, dungeon: &Dungeon) {
    window.erase();

    let max_x = window.get_max_x();
    let max_y = window.get_max_y();

    // Draw dungeon map
    let offset_x = 0;
    let offset_y = 0;

    for y in 0..(max_y - 1) {
        let tile_y = offset_y + y;

        if tile_y < 0 || tile_y as usize >= dungeon.get_height() {
            continue;
        }

        for x in 0..(max_x - 1) {
            let tile_x = offset_x + x;

            if tile_x < 0 || tile_x as usize >= dungeon.get_width() {
                continue;
            }

            let actual_tile = dungeon.get_at(tile_x as usize, tile_y as usize);
            window.mvprintw(y, x, &actual_tile.to_string()); // do i really need a to_string
        }
    }


    // Draw window border/UI
    let k_border = "*";

    for y in 0..max_y {
        window.mvprintw(y, 0, k_border);
        window.mvprintw(y, max_x - 1, k_border);
    }

    for x in 0..max_x {
        window.mvprintw(0, x, k_border);
        window.mvprintw(max_y - 1, x, k_border);
    }

    window.mvprintw(max_y - 1, 3, "HP: 10/10");
}

fn main() {
    let window = initscr();
    window.keypad(true);
    pancurses::curs_set(0); // hide insertion pointer
    pancurses::noecho();

    let dungeon = generate_map();

    loop {
        raster_screen(&window, &dungeon);
        window.refresh();

        match window.getch() {
            //Some(Input::Character(c)) => { window.addch(c); },
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
