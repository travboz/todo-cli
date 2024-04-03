# A Todo cli app built using Rust. 

## Features:
- add tasks - can only add tasks one by one
- mark tasks as completed
- show tasks

## Installation:
To run the app, you can use Cargo: 
```bash
cargo run {commands}
```

### Adding a task:
```bash
cargo run -- add "Buy groceries"
```

### Listing the tasks:
```bash
cargo run -- show
```

### Mark task as complete:
```bash
cargo run -- "Buy groceries" complete
```
