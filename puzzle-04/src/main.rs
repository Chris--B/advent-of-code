#![allow(unused_imports, unused_variables)]

use std::{
    env,
    fs,
    collections,
    io::{
        self,
        BufRead,
    },
    str::FromStr,
};

use failure::bail;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct TimeStamp {
    year:   u32,
    month:  u8,
    day:    u8,
    hour:   u8,
    minute: u8,
}

impl FromStr for TimeStamp {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<TimeStamp, failure::Error> {
        lazy_static::lazy_static! {
            static ref EXPR: Regex = Regex::new(r#"\[(\d+)-(\d+)-(\d+)+ (\d{2}):(\d{2})\]"#).unwrap();
        }
        let caps = EXPR.captures(s).expect("No regex matches?");

        let year:   u32 = caps.get(1)
                              .expect("regex fail on year").as_str().parse()?;
        let month:  u8  = caps.get(2)
                              .expect("regex fail on month").as_str().parse()?;
        let day:    u8  = caps.get(3)
                              .expect("regex fail on day").as_str().parse()?;
        let hour:   u8  = caps.get(4)
                              .expect("regex fail on hour").as_str().parse()?;
        let minute: u8  = caps.get(5)
                              .expect("regex fail on minute").as_str().parse()?;

        assert_eq!(year, 1518);

        Ok(TimeStamp {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Event {
    ShiftStart(TimeStamp, u32),
    FallsAsleep(TimeStamp),
    WakesUp(TimeStamp),
}

impl Event {
    fn timestamp(&self) -> TimeStamp {
        match self {
            Event::ShiftStart(time, _) => *time,
            Event::FallsAsleep(time)   => *time,
            Event::WakesUp(time)       => *time,
        }
    }
}

impl FromStr for Event {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Event, failure::Error> {
        let time: TimeStamp = s.parse()?;
        if s.contains("falls") {
            Ok(Event::FallsAsleep(time))
        } else if s.contains("wakes") {
            Ok(Event::WakesUp(time))
        } else {
            lazy_static::lazy_static! {
                static ref EXPR: Regex = Regex::new(r#"Guard #(\d+)"#).unwrap();
            }

            let id = EXPR.captures(s).unwrap()
                         .get(1).unwrap().as_str()
                         .parse()?;
            Ok(Event::ShiftStart(time, id))
        }
    }
}

fn main() {
    let run = env::args().nth(1).unwrap_or("2".to_string());
    if run == "1" {
        match run1() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    } else if run == "2" {
        match run2() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    let mut events: Vec<Event> = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();
    events.sort_by_key(|e| e.timestamp());

    println!("Found {} events", events.len());

    if false {
        let mut id = match events[0] {
            Event::ShiftStart(_, id) => id,
            _ => bail!("No starting shift?"),
        };
        for event in events.iter() {
            if let Event::ShiftStart(_, new_id) = event {
                id = *new_id;
            }
            println!("#{:<4} {:>02}-{:>02}",
                     id,
                     event.timestamp().hour,
                     event.timestamp().minute);
        }
        println!("");
    }

    let mut asleep_map = collections::HashMap::new();
    let mut id = match events[0] {
        Event::ShiftStart(_, id) => id,
        _ => bail!("No starting shift?"),
    };
    let mut last_minute = 0;
    for event in events.iter() {
        if let Event::ShiftStart(_, new_id) = event {

            id = *new_id;
        }
        let minute: usize;
        if event.timestamp().hour == 23 {
            minute = 0;
        } else {
            minute = event.timestamp().minute as usize;
        };

        let minutes = asleep_map.entry(id).or_insert([0u32; 60]);
        match event {
            Event::ShiftStart(..)  => {
                // Awake
            },
            Event::FallsAsleep(..) => {
                last_minute = minute;
            },
            Event::WakesUp(..)     => {
                for m in last_minute..minute {
                    minutes[m] += 1;
                }
            },
        }
    }

    let sleepy_guard = asleep_map.iter()
        .max_by_key::<u32, _>(|(k, v)| v.iter().sum()).unwrap()
        .0;
    let best_minute = asleep_map
        .get(&sleepy_guard).unwrap()
        .iter()
        .enumerate()
        .max_by_key(|(i, count)| *count).unwrap()
        .0 as u32;
    println!("Guard #{} @ minute {}", sleepy_guard, best_minute);
    println!("          {}",
             asleep_map.get(&sleepy_guard).unwrap()
                .iter()
                .map(|count| format!(" {:>2}", count))
                .collect::<String>());

    println!("Final: {}", sleepy_guard * best_minute);

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    let mut events: Vec<Event> = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();
    events.sort_by_key(|e| e.timestamp());

    println!("Found {} events", events.len());

    if false {
        let mut id = match events[0] {
            Event::ShiftStart(_, id) => id,
            _ => bail!("No starting shift?"),
        };
        for event in events.iter() {
            if let Event::ShiftStart(_, new_id) = event {
                id = *new_id;
            }
            println!("#{:<4} {:>02}-{:>02}",
                     id,
                     event.timestamp().hour,
                     event.timestamp().minute);
        }
        println!("");
    }

    let mut asleep_map = collections::HashMap::new();
    let mut id = match events[0] {
        Event::ShiftStart(_, id) => id,
        _ => bail!("No starting shift?"),
    };
    let mut last_minute = 0;
    for event in events.iter() {
        if let Event::ShiftStart(_, new_id) = event {

            id = *new_id;
        }
        let minute: usize;
        if event.timestamp().hour == 23 {
            minute = 0;
        } else {
            minute = event.timestamp().minute as usize;
        };

        let minutes = asleep_map.entry(id).or_insert([0u32; 60]);
        match event {
            Event::ShiftStart(..)  => {
                // Awake
            },
            Event::FallsAsleep(..) => {
                last_minute = minute;
            },
            Event::WakesUp(..)     => {
                for m in last_minute..minute {
                    minutes[m] += 1;
                }
            },
        }
    }

    let (best_id, best_minute, best_count) = asleep_map
        .iter()
        .map(|(id, minutes)| {
            let (minute, count) = minutes.iter()
                .enumerate()
                .max_by_key(|(i, count)| *count).unwrap();
            (id, minute as u32, count)
        })
        .max_by_key(|(id, minute, count)| *count).unwrap();
    println!("Guard #{} @ minute {}", best_id, best_minute);
    println!("Final: {}", best_id * best_minute);
    Ok(())
}
