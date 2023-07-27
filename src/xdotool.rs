use std::{error::Error, process::Command};

#[derive(Debug)]
pub struct Window(pub u32);

impl Window {
    pub fn find_by_pid(pid: u32) -> Result<Window, Box<dyn Error>> {
        let window_id = dbg!(run_xdotool(&["search", "--pid", pid.to_string().as_str()])?);

        Ok(Window(window_id.trim().parse()?))
    }

    pub fn resize(&self, x1: u16, y1: u16, x2: u16, y2: u16) -> Result<(), Box<dyn Error>> {
        let width = x2 - x1;
        let height = y2 - y1;

        run_xdotool(&[
            "windowsize",
            self.0.to_string().as_str(),
            width.to_string().as_str(),
            height.to_string().as_str(),
        ])?;

        run_xdotool(&[
            "windowmove",
            self.0.to_string().as_str(),
            x1.to_string().as_str(),
            y1.to_string().as_str(),
        ])?;

        Ok(())
    }

    pub fn hide(&self) -> Result<(), Box<dyn Error>> {
        run_xdotool(&["windowunmap", self.0.to_string().as_str()])?;
        Ok(())
    }

    pub fn unhide(&self) -> Result<(), Box<dyn Error>> {
        run_xdotool(&["windowmap", self.0.to_string().as_str()])?;
        Ok(())
    }

    pub fn set_visible(&self, visible: bool) -> Result<(), Box<dyn Error>> {
        match visible {
            true => self.unhide()?,
            false => self.hide()?,
        };
        Ok(())
    }
}

pub fn run_xdotool(args: &[&str]) -> Result<String, Box<dyn Error>> {
    dbg!(args);
    let output = Command::new("xdotool").args(args).output()?;

    if let Some(code) = output.status.code() {
        if code != 0 {
            return Err("xdotool failed to run".into());
        }
    }

    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}
