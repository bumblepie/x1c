use indoc::indoc;
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use xcom_1_card::{generate_timed_phase_prompts, PanicLevel};

fn main() {
    let rng = &mut thread_rng();
    let mut prompts = generate_timed_phase_prompts(0, PanicLevel::Yellow, 0, rng);
    let keep_playing = true;
    let mut round = 0;
    while keep_playing {
        println!("Round {}", round + 1);
        for prompt in prompts.iter() {
            print!("{:?}", prompt);
            stdout().flush().unwrap();
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
        }
        println!(indoc! {"
            Resolution Phase:
            - Audit spending
            - Resolve research
            - Resolve UFO defense
            - Gain continent bonuses or increase panic
            - Purchase replacement satellites/interceptors
        "
        });
        let mut keep_playing_response: Option<bool> = None;
        while keep_playing_response.is_none() {
            println!("Keep Playing? [Y]es/[N]o");
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            keep_playing_response = match buffer.to_ascii_lowercase().trim_end() {
                "y" | "yes" => Some(true),
                "n" | "no" => Some(false),
                _ => None,
            };
        }
        if keep_playing_response.unwrap() == false {
            break;
        }

        let mut panic_response: Option<PanicLevel> = None;
        while panic_response.is_none() {
            println!("What is the current Global Panic Level? [Y]ellow/[O]range/[R]ed");
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            panic_response = match buffer.to_ascii_lowercase().trim_end() {
                "y" | "yellow" => Some(PanicLevel::Yellow),
                "o" | "orange" => Some(PanicLevel::Orange),
                "r" | "red" => Some(PanicLevel::Red),
                _ => None,
            };
        }

        let mut ufos_response: Option<u32> = None;
        while ufos_response.is_none() {
            println!("How many ufos were left on the world map?");
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            ufos_response = match buffer.trim_end().parse::<u32>() {
                Ok(n) => Some(n),
                _ => None,
            };
        }
        println!("Remove UFOs from world map and return interceptors to reserves. Refresh researched technology");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        round += 1;
        prompts = generate_timed_phase_prompts(
            round,
            panic_response.unwrap(),
            ufos_response.unwrap(),
            rng,
        );
    }
}
