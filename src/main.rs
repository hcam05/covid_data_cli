use std::io;

#[macro_use]
extern crate serde;
extern crate reqwest;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

#[derive(Debug, Deserialize)]
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

#[macro_use]
extern crate maplit;

async fn get_data(state_name: String) -> Result<Vec<State>, reqwest::Error> {
    let url = format!("https://corona.lmao.ninja/v2/states?sort=cases");

    let response = reqwest::get(&url).await?;
    let states: Vec<State> = response.json().await?;

    let state: Vec<State> = states
        .into_iter()
        .filter(|state| state.state == state_name)
        .collect();

    Ok(state)
}

fn format_state_name(name: &mut String) -> String {
    let _state_lookup = hashmap! {
        "alabama" => "Alabama",
        "alaska" => "Alaska",
        "arizona" => "Arizona",
        "arkansas" => "Arkansas",
        "california" => "California",
        "colorado" => "Colorado",
        "connecticut" => "Connecticut",
        "delaware" => "Delaware",
        "florida" => "Florida",
        "georgia" => "Georgia",
        "hawaii" => "Hawaii",
        "idaho" => "Idaho",
        "illinois" => "Illinois",
        "indiana" => "Indiana",
        "iowa" => "Iowa",
        "kansas" => "Kansas",
        "kentucky" => "Kentucky",
        "louisiana" => "Louisiana",
        "maine" => "Maine",
        "maryland" => "Maryland",
        "massachusetts" => "Massachusetts",
        "michigan" => "Michigan",
        "minnesota" => "Minnesota",
        "mississippi" => "Mississippi",
        "missouri" => "Missouri",
        "montana" => "Montana",
        "nebraska" => "Nebraska",
        "nevada" => "Nevada",
        "new_hampshire" => "New Hampshire",
        "new_jersey" => "New Jersey",
        "new_mexico" => "New Mexico",
        "new_york" => "New York",
        "north_carolina" => "North Carolina",
        "north_dakota" => "North Dakota",
        "ohio" => "Ohio",
        "oklahoma" => "Oklahoma",
        "oregon" => "Oregon",
        "pennsylvania" => "Pennsylvania",
        "rhode_island" => "Rhode Island",
        "south_carolina" => "South Carolina",
        "south_dakota" => "South Dakota",
        "tennessee" => "Tennessee",
        "texas" => "Texas",
        "utah" => "Utah",
        "vermont" => "Vermont",
        "virginia" => "Virginia",
        "washington" => "Washington",
        "west_virginia" => "West Virginia",
        "wisconsin" => "Wisconsin",
        "wyoming" => "Wyoming",
    };

    let clean_input = name
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("_")
        .to_lowercase();

    let result = match _state_lookup.get::<str>(&clean_input) {
        Some(state_name) => state_name.to_string(),
        None => "not found".to_string(),
    };

    return result;
}

fn enter_state_name() -> String {
    println!("Enter state");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut state_name = format_state_name(&mut input);
    println!("{:?}", input);
    println!("{:?}", state_name);

    if state_name == "not found".to_string() {
        state_name = enter_state_name();
    }

    return state_name;
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let state_name = enter_state_name();
    let state = get_data(state_name).await?;

    if state.len() > 0 {
        let State {
            state,
            today_cases,
            cases,
            deaths,
            active,
            tests,
            ..
        } = &state[0];

        // Create table
        let mut table = Table::new();
        table.add_row(row![bFg -> "Covid Cases", brFb -> state]);
        table.add_row(row!["New Cases", r -> today_cases]);
        table.add_row(row!["Total Cases", r -> cases]);
        table.add_row(row!["Deaths", r -> deaths]);
        table.add_row(row!["Active", r -> active]);
        table.add_row(row!["Tests", r -> tests]);
        table.printstd();
    }

    Ok(())
}
