# Habitron - A CLI Habit Tracker
Nothing fancy, Just a simple habit tracking application on the cli I made while learning `Rust`.
It incorporates all the basic habit tracking concepts like
- adding habits
- counting habits
- removing streaks on a day gap
- Saving the habits to a external json file.
- Marking the habits daily as done

1. Setup and Dependencies:
- `clap` for command-line argument parsing.
- `chrono` for date and time handling.
- `serde` and `serde_json` for serialization and deserialization.

2. CLI Structure:
- `Cli` struct parses the command-line arguments.
- `Commands` enum defines the possible subcommands (`Add`, `Done`, `List`).

3. Habit Management:
- `Habit` struct represents a habit with its name, streak, and the last done date.
- `load_habits` function loads habits from a JSON file.
- `save_habits` function saves habits to a JSON file.

4. Subcommand Handling:
- `Add`: Adds a new habit if it doesn't already exist.
- `Done`: Marks a habit as done for today, increments the streak if it's a new day.
- `List`: Lists all the habits with their current streaks and last done dates.

5. Running the CLI:
- Add a habit: `cargo run -- add "Exercise"`
- Mark a habit as done: `cargo run -- done "Exercise"`
- List habits: `cargo run -- list`
