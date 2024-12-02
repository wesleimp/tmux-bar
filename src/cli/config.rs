use crate::colors::{Color, Style};
use crate::icons::Icon;

use tmux_bar_lib::module::{Module, StyledModule};
use tmux_bar_lib::system::battery::BatteryInformation;

pub fn parse_modules() -> Vec<StyledModule> {
    let battery_information = BatteryInformation::new();
    let battery_percentage = battery_information.map(|x| x.percentages);
    let is_charging = battery_information.map(|x| x.is_charging).unwrap_or(true);

    let battery_icon = Icon::new_battery(&battery_information);

    vec![
        Some(StyledModule::new(
            Module::Cpu(2),
            Some(Icon::Cpu),
            Style {
                fg: Color::Cyan,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Some(StyledModule::new(
            Module::Memory(2),
            Some(Icon::Server),
            Style {
                fg: Color::Yellow,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Some(StyledModule::new(
            Module::Battery,
            battery_icon,
            Style {
                fg: Color::Green,
                bg: Color::Reset,
                bold: false,
            },
        )),
        maybe_insert(
            StyledModule::new(
                Module::Manual("  LOW BATTERY  "),
                None,
                Style {
                    fg: Color::Black,
                    bg: Color::Red,
                    bold: true,
                },
            ),
            battery_percentage.unwrap_or(100) < 20 && !is_charging,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn maybe_insert<T>(something: T, enabled: bool) -> Option<T> {
    if enabled {
        Some(something)
    } else {
        None
    }
}
