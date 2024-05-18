use chrono::{Datelike, Local, NaiveDateTime};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Parser, Debug)]
#[command(name = "Habitron", version = "0.1.0", author = "Tejas Mahajan")]
#[command(about = "A simple CLI habit tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { name: String },
    Done { name: String },
    List,
}

#[derive(Serialize, Deserialize, Debug)]
struct Habit {
    name: String,
    streak: u32,
    last_done: NaiveDateTime,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let mut habits = load_habits("habits.json")?;

    match args.command {
        Commands::Add { name } => {
            if !habits.iter().any(|habit| habit.name == name) {
                habits.push(Habit {
                    name: name.clone(),
                    streak: 0,
                    last_done: Local::now().naive_local(),
                });
                println!("Added habit: {}", name);
            } else {
                println!("Habit already exists.");
            }
        }
        Commands::Done { name } => {
            if let Some(habit) = habits.iter_mut().find(|habit| habit.name == name) {
                let today = Local::now().naive_local();
                if habit.last_done.date() != today.date() || habit.streak == 0 {
                    habit.streak += 1;
                    habit.last_done = today;
                    println!(
                        "Good job! You've done '{}' for {} days in a row!",
                        name, habit.streak
                    );
                } else {
                    println!("You've already done '{}' today.", name);
                }
            } else {
                println!("Habit not found.");
            }
        }
        Commands::List => {
            if habits.is_empty() {
                println!("No habits found. Add some habits first.");
            } else {
                for habit in &habits {
                    println!(
                        "Habit: {}, Streak: {}, Last done: {}",
                        habit.name, habit.streak, habit.last_done
                    );
                }
            }
        }
    }

    save_habits("habits.json", &habits)?;
    Ok(())
}

fn load_habits(filename: &str) -> io::Result<Vec<Habit>> {
    let file = File::open(filename).unwrap_or_else(|_| File::create(filename).unwrap());
    let reader = BufReader::new(file);
    let habits = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);

    // set all the habit streaks to zero where the last done date is not yesterday or today-lastdone > 1
    let today = Local::now().naive_local();

    // let dt: NaiveDateTime =
    // NaiveDateTime::parse_from_str("2021-10-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

    // println!("{}" , today.date().day() - dt.date().day());

    let habits = habits
        .into_iter()
        .map(|mut habit: Habit| {
            if (today.date().day() - habit.last_done.date().day()) > 1 {
                habit.streak = 0;
            }
            habit
        })
        .collect();
    
    Ok(habits)
}

fn save_habits(filename: &str, habits: &Vec<Habit>) -> io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer(file, &habits)?;
    Ok(())
}
