# tonkeytype

A monkeytype-inspired typing trainer for the terminal, written in Rust.

## Install

```sh
cargo install --path .
```

Or run directly:

```sh
cargo run --release
```

## Controls

| Key | Action |
|---|---|
| `arrows` / `j` `k` | Navigate menu |
| `tab` | Cycle theme |
| `enter` | Start test / restart |
| `esc` | Back to menu |
| `ctrl+c` | Quit |
| `ctrl+r` | Restart test |
| `backspace` | Delete last character |

## Modes

- **Timed**: 15, 30, 60, 120 seconds
- **Word count**: 25, 50, 100 words

## Themes

- `dark` (default)
- `light`
- `nord`
- `catppuccin`

Config saved at `~/.config/tonkeytype/config.toml`.

## Stats

- **WPM** — net words per minute (correct chars / 5 / minutes)
- **Raw WPM** — gross keystrokes per minute
- **Accuracy** — correct keystrokes / total keystrokes
