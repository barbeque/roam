extern crate roam;
extern crate pancurses;
use pancurses::{initscr, endwin, Input, noecho};

fn raster_screen(window: &pancurses::Window) {
    window.erase();

    let max_x = window.get_max_x();
    let max_y = window.get_max_y();

    for y in 0..max_y {
        window.mvprintw(y, 0, "#");
        window.mvprintw(y, max_x - 1, "#");
    }

    for x in 0..max_x {
        window.mvprintw(0, x, "#");
        window.mvprintw(max_y - 1, x, "#");
    }
}

fn main() {
    let window = initscr();
    window.printw("Hello, Rust");
    window.keypad(true);
    noecho();

    loop {
        raster_screen(&window);
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
