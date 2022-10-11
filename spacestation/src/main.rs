use inquire::Select;
use inquire::Text;
use rand::{random, thread_rng, Rng};
use rand_derive2::RandGen;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

fn main() {
    let mut station = Station::new();
    let mut station_log = vec![];

    println!("Space Station Named: {} {}", station.name, station.version);
    println!(
        "We have these working sections: {:?}",
        station.working_sections()
    );
    println!(
        "We have these broken sections: {:?}",
        station.broken_sections()
    );
    station_log.push(format!(
        "Space Station Named: {} {}",
        station.name, station.version
    ));
    station_log.push(format!(
        "We have these working sections: {:?}",
        station.working_sections()
    ));
    station_log.push(format!(
        "We have these broken sections: {:?}",
        station.broken_sections()
    ));
    loop {
        // main game loop!
        let days_left = station.days_left();
        if days_left < 1 {
            station_log.push("(END-TRANSMISSION)".to_string());
            println!("(END-TRANSMISSION)");
            save_log(&station_log, &mut station);
            break;
        }
        print!("Day {0}: ", station.day);
        println!(
            "{days_left} WORKING SECTIONS REMAIN {:?}",
            station.working_sections()
        );
        station_log.push(format!(
            "Day {0}: {1}",
            station.day,
            Text::new("Enter your log:").prompt().unwrap()
        ));
        match menu(&["NEW DAY".into(), "STATUS".into(), "POWERDOWN".into()]).as_str() {
            "NEW DAY" => {
                station_log.push(station.new_day());
                match menu(&["REPAIR".into(), "SCIENCE".into()]).as_str() {
                    "REPAIR" => {
                        println!("FIX FIX FIX"); // TODO implement a fixing bit
                        repair(
                            menu(&station.broken_sections()),
                            &mut station,
                            &mut station_log,
                        );
                        continue;
                    }
                    "SCIENCE" => {
                        println!("I am a nerd beep boop"); // TODO implement a science studying bit
                        science(
                            menu(&station.working_sections()),
                            &mut station,
                            &mut station_log,
                        );
                        continue;
                    }
                    &_ => panic!(),
                }
            }
            "STATUS" => station.status(),
            "POWERDOWN" => {
                save_log(&station_log, &mut station);
                break;
            }
            &_ => panic!("RADIATION ANOMALY. DEATH IMMINENT"),
        }
    }
    dbg!(station_log);
}

fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec()).prompt().unwrap()
}

fn science(working_section: String, station: &mut Station, station_log: &mut Vec<String>) {
    println!("Bleep bloop doing science in: {}", working_section);
    station_log.push(format!("Bleep bloop doing science in: {}", working_section));
    station_log.push(station.break_something());
}

fn repair(broken_section: String, station: &mut Station, station_log: &mut Vec<String>) {
    let section = SectionName::from_str(broken_section.as_str()).unwrap();

    let broken_index = station
        .sections
        .iter()
        .position(|m| m.name == section)
        .expect("Section not found.");

    station.sections[broken_index].active = true;
    station_log.push(format!("Bleep bloop doing fixing in: {}", section));
}

fn save_log(station_log: &[String], station: &mut Station) {
    let mut file = OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(format!("{}{}.txt", station.name, station.version))
        .unwrap();

    for log in station_log.iter() {
        let write_result = file.write(format!("{}\n", log).as_bytes());
        println!("{}", write_result.is_ok());
    }
}

#[derive(Debug, RandGen)]
struct Station {
    name: Name,
    version: u8,
    sections: Vec<Section>,
    day: u16,
}

#[derive(Debug, RandGen, Display)]
enum Name {
    Gary,
    Spooky,
    Esme,
    Eisenberg,
    Intrepid,
    Nova,
    Reliant,
}

impl Station {
    fn new() -> Self {
        let mut rng = thread_rng();
        Station {
            name: random(),
            version: random(),
            sections: (0..10).map(|_| random()).collect(),
            day: rng.gen_range(1..3000),
        }
    }

    fn new_day(&mut self) -> String {
        self.day += 1;
        self.break_something()
    }

    fn days_left(&self) -> usize {
        self.sections.iter().filter(|m| m.active).count()
    }

    fn working_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    fn broken_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| !m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    fn break_something(&mut self) -> String {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            let break_it = Self::is_successful();
            if break_it {
                broken_section.active = false;
                println!("(Section-FAILURE {})", &broken_section.name);
                format!("(Section-FAILURE {})", &broken_section.name)
            } else {
                println!("(sections OK)");
                "(sections OK)".to_string()
            }
        } else {
            println!("(sections OK)");
            "(sections OK)".to_string()
        }
    }

    fn is_successful() -> bool {
        thread_rng().gen_bool(0.5)
    }

    fn status(&self) {
        dbg!(&self);
    }
}

#[derive(Debug, RandGen, Eq, PartialEq)]
struct Section {
    name: SectionName,
    active: bool,
}

#[derive(Debug, RandGen, Display, Eq, PartialEq, EnumString)]
enum SectionName {
    AstroScience,
    SolarPanels,
    Antenna,
    Radiators,
    Quarters,
    NuclearGenerator,
    Galley,
    Transponder,
    Tracking,
}
