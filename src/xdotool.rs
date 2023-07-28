use std::{error::Error, process::Command, thread::sleep, time::Duration};

use clap::ValueEnum;
use strum::Display;

#[derive(Display, Clone, ValueEnum)]
pub enum WindowState {
    Modal,
    Sticky,
    MaximizedVert,
    MaximizedHorz,
    Above,
    Below,
    SkipTaskbar,
    SkipPager,
    Fullscreen,
    Hidden,
    Shaded,
    DemandsAttention,
}

#[derive(Debug)]
pub struct Window(pub u32);

impl Window {
    pub fn find_by_pid(pid: u32) -> Result<Window, Box<dyn Error>> {
        let window_id = run_xdotool(&["search", "--pid", pid.to_string().as_str()], false)?;

        Ok(Window(window_id.trim().parse()?))
    }

    pub fn resize(&self, x1: u16, y1: u16, x2: u16, y2: u16) -> Result<(), Box<dyn Error>> {
        let width = x2 - x1;
        let height = y2 - y1;

        run_xdotool(
            &[
                "windowsize",
                self.0.to_string().as_str(),
                width.to_string().as_str(),
                height.to_string().as_str(),
            ],
            true,
        )?;

        run_xdotool(
            &[
                "windowmove",
                self.0.to_string().as_str(),
                x1.to_string().as_str(),
                y1.to_string().as_str(),
            ],
            true,
        )?;

        Ok(())
    }

    pub fn hide(&self) -> Result<(), Box<dyn Error>> {
        run_xdotool(&["windowunmap", self.0.to_string().as_str()], true)?;
        Ok(())
    }

    pub fn unhide(&self) -> Result<(), Box<dyn Error>> {
        run_xdotool(&["windowmap", self.0.to_string().as_str()], true)?;
        Ok(())
    }

    pub fn set_visible(&self, visible: bool) -> Result<(), Box<dyn Error>> {
        match visible {
            true => self.unhide()?,
            false => self.hide()?,
        };
        Ok(())
    }

    pub fn add_window_state(&self, window_state: WindowState) -> Result<(), Box<dyn Error>> {
        run_xdotool(
            &[
                "windowstate",
                "--add",
                window_state.to_string().as_str(),
                self.0.to_string().as_str(),
            ],
            true,
        )?;
        Ok(())
    }

    pub fn remove_window_state(&self, window_state: WindowState) -> Result<(), Box<dyn Error>> {
        run_xdotool(
            &[
                "windowstate",
                "--remove",
                window_state.to_string().as_str(),
                self.0.to_string().as_str(),
            ],
            true,
        )?;
        Ok(())
    }
}

fn _run_xdotool(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("xdotool").args(args).output()?;

    if let Some(code) = output.status.code() {
        let stderr = std::str::from_utf8(&output.stderr)?;
        let stdout = std::str::from_utf8(&output.stderr)?;
        if code != 0 {
            return Err(
                format!("xdotool failed to run:\nSTDOUT:\n{stdout}\n\nSTDERR:\n{stderr}").into(),
            );
        }
    }

    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}

/// Attempt to run Xdotool with up to 5 retries
pub fn run_xdotool(args: &[&str], retry: bool) -> Result<String, Box<dyn Error>> {
    let mut counter = 0;

    loop {
        match _run_xdotool(args) {
            Ok(output) => return Ok(output),
            Err(err) => {
                if counter >= 5 || !retry {
                    return Err(err);
                }
                counter += 1;
                dbg!("Retrying");
                sleep(Duration::from_millis(50))
            }
        }
    }
}
