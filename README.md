# WhatDoWe-Now?
WhatDoWe-Now is a Rust Todo list that runs entirely on the terminal!
 
## Installation
 
Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, then run:
 
```bash
cargo install --git https://github.com/Raagranato/WhatDoWeDoNow
```
 
That's it! The `wdwn` command will be available globally.
 
## Usage
 
```bash
wdwn
```
 
### Commands
 
| Command | Description | Example |
|---|---|---|
| `add [task]` | Add a new task | `add Buy groceries` |
| `done [task]` | Mark a task as done | `done groceries` |
| `remove [task]` | Remove a task | `remove groceries` |
| `list` | List all tasks | `list` |
| `delete` | Delete all tasks | `delete` |
| `clear` | Clear the terminal | `clear` |
| `menu` | Show available commands | `menu` |
| `exit` | Exit the program | `exit` |
 
### Smart search
Commands like `done` and `remove` accept partial matches — you don't need to type the full task name. If multiple tasks match, the app will list them and ask you to be more specific.
 
## Features
 
- Partial task name matching for `done` and `remove`
- Completed tasks shown with ~~strikethrough~~ in green
- Task counter showing completed/total tasks
- Delete confirmation prompt
- Cross-platform (Windows and Linux)

## Built with
 
- [Rust](https://www.rust-lang.org/)
- [colored](https://crates.io/crates/colored) — terminal colors
 
