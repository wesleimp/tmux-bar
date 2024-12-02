use crate::colors;
use crate::formatter;
use crate::icons;
use crate::system::{battery::BatteryInformation, cpu};

use chrono::{DateTime, Local};
use std::time::Duration;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

// Those are only constructed in config.rs
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Module {
    Manual(&'static str),
    Time(&'static str),
    Battery,
    Cpu(usize),
    Memory(usize),
    Swap(usize),
    Uptime,
    SessionName,
    WindowName,
    WindowIndex,
    PaneIndex,
    Hostname,
}

impl Module {
    fn display(self) -> Result<String, ()> {
        match self {
            Module::Manual(s) => Ok(String::from(s)),
            Module::Time(format) => {
                let now: DateTime<Local> = Local::now();

                Ok(now.format(format).to_string())
            }
            Module::Battery => {
                BatteryInformation::new().map(|info| format!("{}%", info.percentages))
            }
            Module::SessionName => Ok(String::from("#S")),
            Module::WindowName => Ok(String::from("#W")),
            Module::WindowIndex => Ok(String::from("#I")),
            Module::PaneIndex => Ok(String::from("#P")),
            Module::Hostname => Ok(String::from("#H")),
            Module::Cpu(rounding) => Ok(formatter::round(cpu::get_total_average(), rounding)),
            Module::Memory(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_memory = sys.total_memory();
                let used_memory = sys.used_memory();

                let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
                Ok(formatter::round(memory_usage_percent, rounding))
            }
            Module::Uptime => {
                let uptime = System::uptime();
                let uptime = Duration::from_secs(uptime);

                Ok(format!("{}", formatter::PrettyDuration::new(uptime)))
            }
            Module::Swap(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_swap = sys.total_swap();
                let used_swap = sys.used_swap();

                let swap_usage_percent = (used_swap as f64 / total_swap as f64) * 100.0;
                Ok(formatter::round(swap_usage_percent, rounding))
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct StyledModule {
    module: Module,
    icon: Option<icons::Icon>,
    style: colors::Style,
}

impl StyledModule {
    pub fn new(module: Module, icon: Option<icons::Icon>, style: colors::Style) -> Self {
        Self {
            module,
            icon,
            style,
        }
    }

    pub fn display(&self) -> Result<String, ()> {
        let content = self.module.display()?;

        if let Some(icon) = self.icon {
            Ok(format!(
                "{}{} {}{}",
                self.style.display(),
                icon,
                content,
                colors::Style::default().display()
            ))
        } else {
            Ok(format!(
                "{}{}{}",
                self.style.display(),
                content,
                colors::Style::default().display()
            ))
        }
    }
}

pub fn pre_modules() -> &'static str {
    ""
}

pub fn post_modules() -> &'static str {
    " "
}

pub fn between_modules() -> &'static str {
    " "
}