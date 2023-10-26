mod window;

use cosmic::app::{run, Settings};
use std::error::Error;
use window::Window;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::default();

    run::<Window>(settings, ())?;

    Ok(())
}