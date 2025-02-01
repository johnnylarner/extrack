use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use chrono::Local;

use crate::cmd::Action;

pub fn run_track(action: &Action) {
    let track: Track = action.into();

    println!("Welcome to Extrack");
    println!("Please begin tracking your expenses");
    let _ = std::io::stdout().flush();
    let expense_components = ["Date", "Amount", "Lobel", "Category"];
    let mut inputs: Vec<String> = Vec::with_capacity(expense_components.len());
    for comp in expense_components.iter() {
        print!("{comp}: ");
        let _ = std::io::stdout().flush();
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer).unwrap();
        inputs.push(buffer.trim().into());
    }
    let acc = String::new();
    let output = expense_components
        .iter()
        .zip(inputs)
        .fold(acc, |acc, (comp, inp)| format!("{acc}{comp}: {inp}, "));
    track.write_expenses(&output);
    let _ = std::io::stdout().flush();
}

#[derive(Debug, Clone)]
struct Track;

impl Track {
    fn out_path(&self) -> PathBuf {
        let mut current_dir = std::env::current_dir().unwrap();
        let file_name = format!("expenses_{}", Local::now().format("%Y-%m-%d_%H:%M:%S"));
        current_dir.push(file_name);
        current_dir
    }
    pub fn write_expenses(&self, expenses: &str) {
        let out = self.out_path();
        let out_str = out.to_str().unwrap().to_string();
        let file = File::create(out).unwrap();
        let mut writer = BufWriter::new(file);

        writer.write_all(expenses.as_bytes()).unwrap();
        println!("Expenses written to {out_str}");
    }
}

impl From<&Action> for Track {
    fn from(value: &Action) -> Self {
        match value {
            Action::Track => Track,
        }
    }
}
