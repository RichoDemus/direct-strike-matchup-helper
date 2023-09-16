use std::collections::HashMap;
use std::hash::Hash;
use strum::EnumString;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

const _URL: &str = "https://sheets.googleapis.com/v4/spreadsheets/1L6MiZAl0VL3yyxrVOyiQGBY2vY43jeyJd0WWpIanOSk/values/HRP%20Strats(WIP)?key=<KEY>";

#[derive(EnumString, Debug, Default, Serialize)]
pub enum Outcome {
    Even,
    Favorable,
    #[strum(serialize="Hard countered", serialize="Hard Countered")]
    HardCountered,
    #[strum(serialize="Hard counter", serialize="Hard Counter")]
    HardCounter,
    Unfavorable,
    #[strum(serialize="")]
    #[default]
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Hash, EnumString, Serialize)]
pub enum Commander {
    Alarak,
    Abathur,
    Artanis,
    Dehaka,
    Fenix,
    HanAndHorner,
    Karax,
    Kerrigan,
    Mengsk,
    Nova,
    Raynor,
    Stetmann,
    Stukov,
    Swann,
    Tychus,
    Vorazun,
    Zagara,
}

impl Commander {
    fn from_column(column: usize) -> Option<Commander> {
        match column {
            2 => Some(Commander::Alarak),
            3 => Some(Commander::Abathur),
            4 => Some(Commander::Artanis),
            5 => Some(Commander::Dehaka),
            6 => Some(Commander::Fenix),
            7 => Some(Commander::HanAndHorner),
            8 => Some(Commander::Karax),
            9 => Some(Commander::Kerrigan),
            10 => Some(Commander::Mengsk),
            11 => Some(Commander::Nova),
            12 => Some(Commander::Raynor),
            13 => Some(Commander::Stetmann),
            14 => Some(Commander::Stukov),
            15 => Some(Commander::Swann),
            16 => Some(Commander::Tychus),
            17 => Some(Commander::Vorazun),
            18 => Some(Commander::Zagara),
            _ => None,
        }
    }
    fn from_row(row: usize) -> Option<Commander> {
        match row {
            2 => Some(Commander::Abathur),
            5 => Some(Commander::Alarak),
            8 => Some(Commander::Artanis),
            11 => Some(Commander::Dehaka),
            14 => Some(Commander::Fenix),
            17 => Some(Commander::HanAndHorner),
            20 => Some(Commander::Karax),
            23 => Some(Commander::Kerrigan),
            26 => Some(Commander::Mengsk),
            29 => Some(Commander::Nova),
            32 => Some(Commander::Raynor),
            35 => Some(Commander::Stetmann),
            38 => Some(Commander::Stukov),
            41 => Some(Commander::Swann),
            44 => Some(Commander::Tychus),
            47 => Some(Commander::Vorazun),
            50 => Some(Commander::Zagara),
            _ => None,
        }
    }
}

pub fn get_matchups() -> Result<HashMap<Commander, Vec<Matchup>>, Box<dyn std::error::Error>> {
    let json = include_str!("../../raw.json");
    let resp:SheetRoot = serde_json::from_str(json)?;
    // let resp: SheetRoot = reqwest::get(URL).await?
    //     .json().await?;

    let mut matchups: HashMap<(Commander, Commander), (Outcome, String)> = HashMap::new();

    for row in 0..resp.values.len() {
        let values = &resp.values[row];
        for column in 0..values.len() {
            let cell = &values[column];
            match (Commander::from_row(row), Commander::from_column(column)) {
                (Some(commander), Some(opponent)) => {
                    let matchup = &resp.values[row + 1].get(column);
                    let matchup = matchup.map(|m| Outcome::from_str(m.trim()).expect(format!("can't parse: {m}").as_str()));
                    // println!("{commander:?} vs {opponent:?} : {:?}: {}", matchup, cell.replace("\n", ""));
                    matchups.insert((commander, opponent), (matchup.unwrap_or_default(), cell.to_string()));
                }
                _ => (),
            }

        }
    }

    println!("{:?}", matchups);

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 3 {
    //     eprintln!("Usage: {} <arg1> <arg2>", args[0]);
    // } else {
    //     // Extract the two arguments
    //     let arg1 = &args[1];
    //     let arg2 = &args[2];
    //
    //     // Print the arguments
    //     println!("Argument 1: {}", arg1);
    //     println!("Argument 2: {}", arg2);
    //
    //     println!("{:?}", matchups.get(&(Commander::from_str(arg1).unwrap(), Commander::from_str(arg2).unwrap())));
    // }

    let mut result: HashMap<Commander, Vec<Matchup>> = HashMap::new();

    for ((commander,opponent),(outcome, strategy)) in matchups.into_iter() {
        result.entry(commander).or_default().push(Matchup{opponent, outcome, strategy});
    }

    // let result: HashMap<Commander, Vec<Matchup>> = matchups.into_iter()
    //     .map(|((commander,opponent),(outcome, strategy)) | {
    //         (commander,Matchup{opponent, outcome, strategy})
    //     }).collect();


    Ok(result)
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Matchup {
    opponent: Commander,
    outcome: Outcome,
    strategy: String,
}


#[derive(Debug, Deserialize)]
struct SheetRoot {
    values: Vec<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hard_countered() {
        let matchup = Outcome::from_str("Hard Countered");
        println!("{:?}", matchup);
    }
}