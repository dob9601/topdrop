use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The name of the application to launch
    pub application_name: String,

    /// The top left x coordinate of the window
    pub x1: u16,

    /// The top left y coordinate of the window
    pub y1: u16,

    /// The bottom right x coordinate of the window
    pub x2: u16,

    /// The bottom right y coordinate of the window
    pub y2: u16,

    /// Only needed if trying to manage multiple dropdowns from the same application
    pub unique_name: Option<String>,
}
