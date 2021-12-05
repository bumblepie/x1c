use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use xcom_1_card::{
    generate_timed_phase_prompts, GameResult, PanicLevel, ResolutionPhasePrompt, TimedPhasePrompt,
};

fn prompt_console(input: &str) -> String {
    print!("{}", input);
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    return buffer;
}

enum PanicLevelInput {
    PanicLevel(PanicLevel),
    AlienSpace,
}

fn get_panic_level_input() -> PanicLevelInput {
    let mut panic_response: Option<PanicLevelInput> = None;
    while panic_response.is_none() {
        let panic_level_input = prompt_console(
            "What is the current Global Panic Level? [Y]ellow/[O]range/[R]ed/[A]lien space\n",
        );
        panic_response = match panic_level_input.to_ascii_lowercase().trim_end() {
            "y" | "yellow" => Some(PanicLevelInput::PanicLevel(PanicLevel::Yellow)),
            "o" | "orange" => Some(PanicLevelInput::PanicLevel(PanicLevel::Orange)),
            "r" | "red" => Some(PanicLevelInput::PanicLevel(PanicLevel::Red)),
            "a" | "aliens space" => Some(PanicLevelInput::AlienSpace),
            _ => None,
        };
    }
    return panic_response.unwrap();
}

fn get_ufos_left() -> u32 {
    let mut ufos_response: Option<u32> = None;
    while ufos_response.is_none() {
        let ufos_response_input = prompt_console("How many ufos were left on the world map?\n");
        ufos_response = match ufos_response_input.trim_end().parse::<u32>() {
            Ok(n) => Some(n),
            _ => None,
        };
    }
    return ufos_response.unwrap();
}

fn get_alien_base_destroyed() -> bool {
    let mut alien_base_destroyed: Option<bool> = None;
    while alien_base_destroyed.is_none() {
        let alien_base_destroyed_input =
            prompt_console("Was the alien base destroyed? [Y]es/[N]o\n");
        alien_base_destroyed = match alien_base_destroyed_input.to_ascii_lowercase().trim_end() {
            "y" | "yes" => Some(true),
            "n" | "no" => Some(false),
            _ => None,
        };
    }
    return alien_base_destroyed.unwrap();
}

fn main() {
    let game_result = run_game();
    println!("{:?}", game_result);
}

struct GameState {
    round: u32,
    alien_base_discovered: bool,
    panic_level: PanicLevel,
    ufos_left: u32,
}

fn run_game() -> GameResult {
    let rng = &mut thread_rng();

    let mut game_state = GameState {
        round: 1,
        alien_base_discovered: false,
        panic_level: PanicLevel::Yellow,
        ufos_left: 0,
    };
    loop {
        println!("Round {}", game_state.round);
        prompt_console("Beginning Timed phase");
        let timed_phase_prompts = generate_timed_phase_prompts(
            game_state.round,
            &game_state.panic_level,
            game_state.ufos_left,
            game_state.round == 5,
            rng,
        );
        for prompt in timed_phase_prompts.iter() {
            prompt_console(&format!("{:?}", prompt));
            if let TimedPhasePrompt::AlienBaseDiscovered(_) = prompt {
                game_state.alien_base_discovered = true;
            }
        }
        prompt_console("Ending Timed phase");
        prompt_console("Beginning Resolution phase");
        let mut resolution_phase_prompt = Some(ResolutionPhasePrompt::start());
        while let Some(prompt) = resolution_phase_prompt {
            prompt_console(&format!("{:?}", prompt));
            if prompt == ResolutionPhasePrompt::AskForBoardState {
                let panic_level_input = get_panic_level_input();
                let ufos_left = get_ufos_left();
                let alien_base_destroyed = if game_state.alien_base_discovered {
                    get_alien_base_destroyed()
                } else {
                    false
                };
                match (alien_base_destroyed, panic_level_input) {
                    (true, PanicLevelInput::AlienSpace) => {
                        return GameResult::PyrrhicVictory;
                    }
                    (false, PanicLevelInput::AlienSpace) => {
                        return GameResult::Defeat;
                    }
                    (true, _) => {
                        return GameResult::Victory;
                    }
                    (false, PanicLevelInput::PanicLevel(panic_level)) => {
                        game_state.panic_level = panic_level;
                        game_state.ufos_left = ufos_left;
                    }
                }
            }
            resolution_phase_prompt = prompt.next();
        }
        game_state.round += 1;
    }
}
