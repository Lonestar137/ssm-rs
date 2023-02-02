mod events;
use events::event_handler::{
    is_event_available, 
    wait_for_event
};

use events::event_loop::{
    enter_event_loop
};

use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend, 
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    backend::Backend,
    Frame,
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};




const APPLICATON_TITLE: &str = "SSM";





fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[1]);
}


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    match execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
        Ok(_) => (),
        Err(e) => println!("Error on execute: {}", e)
    };

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(APPLICATON_TITLE)
                .borders(Borders::ALL);

            f.render_widget(block, size);
        })?;

        ui(&mut terminal.get_frame());

        if is_event_available().unwrap() {
            let event = wait_for_event();
            match event {
                Ok(event_to_handle) => {
                    let result = enter_event_loop(event_to_handle);
                    if result.is_err() {
                        format!("{}", result.unwrap_err());
                        break;
                    };

                },
                Err(e) => println!("{}", e),
            }
        }
   }
    
//    thread::sleep(Duration::from_secs(1));

   disable_raw_mode()?;
   execute!(
       terminal.backend_mut(),
       LeaveAlternateScreen,
       DisableMouseCapture
   )?;

   terminal.show_cursor()?;

   Ok(())
}


