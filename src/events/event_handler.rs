use std::time::Duration;
use crossterm::{
    event::{Event, read, poll}
};

pub fn is_event_available() -> crossterm::Result<bool> {
    // Wait for an `Event` availability for 100ms. It returns immediately
    // if an `Event` is/becomes available.
    poll(Duration::from_millis(100))
}

pub fn wait_for_event()-> Result<Event, String> {
    let result = read();
    match result {
        Ok(event) => {
            match event {
                Event::Key(key_event) => Ok(Event::Key(key_event)),
                Event::Mouse(mouse_event) => Ok(Event::Mouse(mouse_event)),
                Event::Paste(string) => Ok(Event::Paste(string)),
                Event::Resize(cols, rows) => Ok(Event::Resize(cols, rows)),
                Event::FocusGained => Ok(Event::FocusGained), 
                Event::FocusLost => Ok(Event::FocusGained)
            }
        },
        Err(e) => return Err(format!("An error ocurred while handling the event: {}", e))
    }
}

