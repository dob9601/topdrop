use std::{error::Error, process::Command, thread::sleep, time::Duration};

use clap::Parser;
use data::ApplicationState;
use xdotool::WindowState;

use crate::{cli::Cli, data::State, xdotool::Window};

pub mod cli;
pub mod data;
pub mod xdotool;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let mut window_states = vec![];

    if args.modal {
        window_states.push(WindowState::Modal);
    }

    if args.sticky {
        window_states.push(WindowState::Sticky);
    }

    if args.maximized_vert {
        window_states.push(WindowState::MaximizedVert);
    }

    if args.maximized_horz {
        window_states.push(WindowState::MaximizedHorz);
    }

    if args.above {
        window_states.push(WindowState::Above);
    }

    if args.below {
        window_states.push(WindowState::Below);
    }

    if args.skip_taskbar {
        window_states.push(WindowState::SkipTaskbar);
    }

    if args.skip_pager {
        window_states.push(WindowState::SkipPager);
    }

    if args.fullscreen {
        window_states.push(WindowState::Fullscreen);
    }

    if args.hidden {
        window_states.push(WindowState::Hidden);
    }

    if args.shaded {
        window_states.push(WindowState::Shaded);
    }

    if args.demands_attention {
        window_states.push(WindowState::DemandsAttention);
    }

    let state_key = args
        .unique_name
        .as_deref()
        .unwrap_or(&args.application_name);

    let mut global_state = State::open()?;
    let maybe_window_state = global_state.data.get_mut(state_key);

    if let Some(state) = maybe_window_state {
        let window = Window(state.id);

        let maybe_err = window.set_visible(state.visible);
        if let Ok(()) = maybe_err {
            state.visible = !state.visible;
            global_state.save()?;
            window.resize(args.x1, args.y1, args.x2, args.y2)?;
            window_states
                .into_iter()
                .map(|state| window.add_window_state(state))
                .collect::<Result<Vec<_>, _>>()?;
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
    window_states
        .into_iter()
        .map(|state| window.add_window_state(state))
        .collect::<Result<Vec<_>, _>>()?;

    global_state
        .data
        .insert(state_key.to_string(), ApplicationState::new(window.0, true));

    global_state.save()?;
    Ok(())
}
