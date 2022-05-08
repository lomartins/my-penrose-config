#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        manager::WindowManager,
        hooks::Hooks,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector
};
use penrose::draw::{Color, dwm_bar, TextStyle};
use penrose::xcb::XcbDraw;

use simplelog::{LevelFilter, SimpleLogger};


// Replace these with your preferred terminal and program launcher
const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "rofi -modi drun,run,filebrowser,window -lines 12 -padding 18 -width 20 -location 0 -show drun -sidebar-mode";

const HEIGHT: usize = 18;
const BLACK: &str = "#282828";
const WHITE: &str = "#ebdbb2";
const GREY: &str = "#3c3836";
const BLUE: &str = "#458588";



fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("Unable to set log level: {}", e);
    };

    let style: TextStyle = TextStyle {
        font: "mono".to_string(),
        point_size: 11,
        fg: Color::try_from(WHITE)?,
        bg: Some(Color::try_from(BLACK)?),
        padding: (2.0, 2.0),
    };

    let config = Config::default();
    let hooks: Hooks<_> = vec![
        Box::new(dwm_bar(
            XcbDraw::new()?,
            HEIGHT,
            &style,
            Color::try_from(BLUE)?, // highlight
            Color::try_from(GREY)?, // empty_ws
            config.workspaces().clone(),
        )?)
    ];


    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-Tab" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit Penrose (important to remember this one!)
        "M-S-Escape" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-c" => run_internal!(kill_client);

        // workspace management
        "A-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}