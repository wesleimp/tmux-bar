mod colors;
mod config;
mod modules;

use clap::Parser;
use modules::styled::StyledModule;
use std::sync::Arc;
use std::thread;
use tmux_bar_lib::icons;

/// TMux Bar Configuration CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opt {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    left: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    right: bool,
}

fn main() {
    let opt = Opt::parse();
    if opt.right {
        print_modules(config::get_right_modules());
        return;
    }

    if opt.left {
        print_modules(config::get_left_modules());
        return;
    }
}

fn print_modules(modules: Vec<StyledModule>) {
    let handles = modules.into_iter().map(|styled_mod| {
        let styled_mod = Arc::new(styled_mod);
        let thread_styled_mod = Arc::clone(&styled_mod);

        thread::spawn(move || thread_styled_mod.display().ok())
    });

    let strings: Vec<_> = handles.filter_map(|t| t.join().ok().flatten()).collect();

    println!(
        "{}{}{}",
        config::pre_modules(),
        strings.join(config::between_modules()),
        config::post_modules(),
    );
}
