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

    /// makes the window fullscreen   
    #[clap(long, action)]
    pub fullscreen: bool,

    /// makes the window into a modal
    #[clap(long, action)]
    pub modal: bool,

    /// makes the window appear on all workspaces
    #[clap(long, action)]
    pub sticky: bool,

    /// sizes the window maximized vertically
    #[clap(long, action)]
    pub maximized_vert: bool,

    /// sizes the window maximized horizontally
    #[clap(long, action)]
    pub maximized_horz: bool,

    /// Show window above all others (always on top)
    #[clap(long, action)]
    pub above: bool,

    /// Show window below all others
    #[clap(long, action)]
    pub below: bool,

    /// hides the window from the taskbar
    #[clap(long, action)]
    pub skip_taskbar: bool,

    /// hides the window from the window pager
    #[clap(long, action)]
    pub skip_pager: bool,

    /// unmaps the window
    #[clap(long, action)]
    pub hidden: bool,

    #[clap(long, action)]
    /// rolls the window up
    pub shaded: bool,

    #[clap(long, action)]
    /// marks window urgent or needing attention
    pub demands_attention: bool,
}
