use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{event, terminal};
use rim::CleanUp;

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    // press Ctrl-Q to quit
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::CONTROL,
                    } => break,
                    _ => {}
                }
                println!("{:?}\r", event);
            }
        } else {
            println!("No input yet\r");
        }
    }
    Ok(())
}
