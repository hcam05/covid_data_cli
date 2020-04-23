#[macro_use]
extern crate serde;
extern crate reqwest;
#[macro_use]
extern crate prettytable;
use prettytable::Table;

use std::io;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct State {
    state: String,
    cases: u32,
    today_cases: u32,
    deaths: u32,
    today_deaths: u32,
    active: u32,
    tests: u32,
    tests_per_one_million: u32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Enter state");
    let mut state_name = String::new();

    // TODO find better way to handle edge case where user doesnt enter anything
    // TODO add more input validation
    io::stdin().read_line(&mut state_name).unwrap();

    let url = format!("https://corona.lmao.ninja/v2/states?sort=cases");

    let response = reqwest::get(&url).await?;
    let states: Vec<State> = response.json().await?;

    let state: Vec<State> = states
        .into_iter()
        .filter(|state| state.state == state_name.trim())
        .collect();

    // TODO is there a way to iterate over a struct?
    if state.len() > 0 {
        let mut table = Table::new();
        table.add_row(row![bFg->"Covid Cases", br -> state[0].state]);
        table.add_row(row!["New Cases", r -> state[0].today_cases]);
        table.add_row(row!["Total Cases", r -> state[0].cases]);
        table.add_row(row!["Deaths", r -> state[0].deaths]);
        table.add_row(row!["Active", r -> state[0].active]);
        table.add_row(row!["Tests", r -> state[0].tests]);
        table.printstd();
    }

    Ok(())
}
