use std::io::{self, Write};
use crossterm::{execute, terminal};

pub fn clear_console(){
    let clear_screen = terminal::Clear(terminal::ClearType::All);
    execute!(io::stdout(), clear_screen).expect("Error al limpiar la consola");
}