# tmux-bar

Simple tmux status line written Rust.

## Features

- Fully configured in Rust
  - Type-save configuration
  - Can be programmed (e.g. dynamically rendered modules)
- Supports coloring
- Multithreaded

## Installation

1. Clone this repository

```bash
git clone git@github.com:wesleimp/tmux-bar.git
```

2. Install Muxbar

```bash
cargo install --path
```

3. Apply Muxbar in your `.tmux.conf`

```conf
set -g status-right '#(tmux-bar)'
```

## Configuration

The configuration is written in Rust and located in `./src/config.rs`
