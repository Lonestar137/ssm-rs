use crossterm::{
    event::{Event, KeyEvent, KeyCode},
};

pub fn enter_event_loop(event: Event) -> Result<Event, String>{
    match event {
        Event::Key(key_event) => return key_event_handler(key_event),
        _ => Err(format!("Event type not handled."))
    }
}

// Event loop for KeyEvent
pub fn key_event_handler(key: KeyEvent) -> Result<Event, String> {
    if key.code == KeyCode::Char('q'){
        return Err(format!("Exit key pressed, exiting program"));
    } else {
        todo!()
    }
}