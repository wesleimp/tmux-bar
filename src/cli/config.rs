use tmux_bar_lib::module::{Module, StyledModule};

pub fn parse_modules() -> Vec<StyledModule> {
    vec![
        Some(StyledModule::new(Module::Cpu(2), Some("CPU"))),
        Some(StyledModule::new(Module::Memory(2), Some("MEM"))),
        Some(StyledModule::new(Module::Battery, Some("BAT"))),
        Some(StyledModule::new(Module::Time("%a %d %b %R"), None)),
    ]
    .into_iter()
    .flatten()
    .collect()
}
