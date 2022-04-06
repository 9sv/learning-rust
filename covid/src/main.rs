use std::collections::HashMap;

fn cap (s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let resp = reqwest::blocking::get("https://disease.sh/v3/covid-19/all")?
        .json::<HashMap<String, f64>>()?;
    for (key, value) in resp {
        let key = cap(key);
        if vec![String::from("OneTestPerPeople"), String::from("OneDeathPerPeople"), String::from("OneCasePerPeople")].contains(&key) {
            println!("{}: {}", key, value as u32 != 0);
        } else {
            println!("{}: {}", key, value);
        }
    }
    Ok(())
}
