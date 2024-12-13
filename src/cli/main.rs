mod config;

use std::sync::Arc;
use std::thread;

use tmux_bar_lib::module;

fn main() {
    let modules = config::parse_modules();
    let handles = modules.into_iter().map(|styled_mod| {
        let styled_mod = Arc::new(styled_mod);
        let thread_styled_mod = Arc::clone(&styled_mod);

        thread::spawn(move || thread_styled_mod.display().ok())
    });

    let strings: Vec<_> = handles.filter_map(|t| t.join().ok().flatten()).collect();

    println!(
        "{}{}{}",
        module::pre_modules(),
        strings.join(module::between_modules()),
        module::post_modules(),
    );
}
