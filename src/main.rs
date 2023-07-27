use std::{error::Error, process::Command, thread::sleep, time::Duration};

use clap::Parser;
use data::WindowState;

use crate::{cli::Cli, data::State, xdotool::Window};

pub mod cli;
pub mod data;
pub mod xdotool;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let state_key = args
        .unique_name
        .as_deref()
        .unwrap_or(&args.application_name);

    let mut global_state = State::open()?;
    let maybe_window_state = global_state.data.get_mut(state_key);

    if let Some(state) = maybe_window_state {
        let window = Window(state.id);

        // window.unhide()?;
        // sleep(Duration::from_millis(5000));
        // window.hide()?;

        let maybe_err = window.set_visible(state.visible);
        if let Ok(()) = maybe_err {
            state.visible = !state.visible;
            global_state.save()?;
            window.resize(args.x1, args.y1, args.x2, args.y2)?;
            return Ok(());
        };
    };
    let pid = Command::new(&args.application_name).spawn()?.id();

    let mut counter = 0;
    let window = loop {
        let maybe_window = Window::find_by_pid(pid);
        if let Ok(window) = maybe_window {
            break window;
        } else if counter < 100 {
            counter += 1;
            sleep(Duration::from_millis(10))
        } else {
            return Err("Too many attempts at finding window".into());
        }
    };

    window.resize(args.x1, args.y1, args.x2, args.y2)?;

    global_state
        .data
        .insert(state_key.to_string(), WindowState::new(window.0, true));

    global_state.save()?;
    Ok(())
}
