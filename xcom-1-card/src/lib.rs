use std::fmt::Display;

use rand::{
    distributions::{Uniform, WeightedIndex},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanicLevel {
    Yellow,
    Orange,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Continent {
    America,
    Africa,
    Eurasia,
}

impl Continent {
    pub fn lowercase(&self) -> String {
        match self {
            Continent::America => "america",
            Continent::Africa => "africa",
            Continent::Eurasia => "eurasia",
        }
        .to_owned()
    }
}

impl Display for &Continent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Continent::America => write!(f, "America"),
            Continent::Africa => write!(f, "Africa"),
            Continent::Eurasia => write!(f, "Eurasia"),
        }
    }
}

pub const ALL_CONTINENTS: [Continent; 3] =
    [Continent::America, Continent::Africa, Continent::Eurasia];

pub fn random_continent<R>(rng: &mut R) -> Continent
where
    R: Rng,
{
    ALL_CONTINENTS.choose(rng).unwrap().clone()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimedPhasePrompt {
    TakeIncome(i32),
    RollUFOLocation(Continent),
    AddUFOsToLocation(Continent, i32),
    SwapUFOLocations(Continent, Continent),
    ChooseResearch,
    AssignInterceptors(Continent),
    AlienBaseDiscovered(Continent),
}

impl TimedPhasePrompt {
    pub fn must_come_after(&self, other: &TimedPhasePrompt) -> bool {
        match self {
            // Can't add to a location until the related die has been rolled
            TimedPhasePrompt::AddUFOsToLocation(location, _) => match other {
                TimedPhasePrompt::RollUFOLocation(other_location) if other_location == location => {
                    true
                }
                _ => false,
            },
            // Let player at least see which die has been rolled for this location before making them assign interceptors
            // Dice might be swapped/added to after interceptors have been assigned
            TimedPhasePrompt::AssignInterceptors(location) => match other {
                TimedPhasePrompt::RollUFOLocation(other_location) if other_location == location => {
                    true
                }
                _ => false,
            },
            // Can't swap locations until the both related dice have been rolled
            TimedPhasePrompt::SwapUFOLocations(location_1, location_2) => match other {
                TimedPhasePrompt::RollUFOLocation(other_location)
                    if other_location == location_1 || other_location == location_2 =>
                {
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn title(&self) -> String {
        match self {
            TimedPhasePrompt::TakeIncome(_) => "Take Income".to_owned(),
            TimedPhasePrompt::RollUFOLocation(continent) => {
                format!("UFOs Spotted Over {:?}", continent)
            }
            TimedPhasePrompt::AddUFOsToLocation(continent, _) => {
                format!("More UFOs Spotted Over {:?}", continent)
            }
            TimedPhasePrompt::SwapUFOLocations(_, _) => format!("UFOs on the Move"),
            TimedPhasePrompt::ChooseResearch => "Choose Research".to_owned(),
            TimedPhasePrompt::AssignInterceptors(continent) => {
                format!("Assign Interceptors to {:?}", continent)
            }
            TimedPhasePrompt::AlienBaseDiscovered(continent) => {
                format!("Alien Base Discovered in {:?}", continent)
            }
        }
    }
}

pub fn generate_timed_phase_prompts<R>(
    round: u32,
    panic: &PanicLevel,
    leftover_ufos: u32,
    discover_alien_base: bool,
    rng: &mut R,
) -> Vec<TimedPhasePrompt>
where
    R: Rng,
{
    let random_income_amounts = [-1, 0, 1];
    let random_income_weights = [0.30, 0.50, 0.20];
    let random_income_dist = WeightedIndex::new(&random_income_weights).unwrap();
    let random_income_adjustment = random_income_amounts[random_income_dist.sample(rng)];
    let income = match panic {
        PanicLevel::Yellow => 6,
        PanicLevel::Orange => 5,
        PanicLevel::Red => 4,
    } + random_income_adjustment;
    let income_prompt = TimedPhasePrompt::TakeIncome(income);

    let mut round_continents = Vec::from(ALL_CONTINENTS.clone());
    round_continents.shuffle(rng);

    // First and second rounds only roll 2 dice
    if round < 3 {
        round_continents.remove(2);
    }

    let mut ufo_prompts: Vec<TimedPhasePrompt> = round_continents
        .iter()
        .map(|continent| TimedPhasePrompt::RollUFOLocation(continent.clone()))
        .collect();

    let mut bonus_ufo_prompts = Vec::new();
    if round >= 2 {
        bonus_ufo_prompts.push(TimedPhasePrompt::AddUFOsToLocation(
            random_continent(rng),
            2,
        ));
    }
    if round >= 3 {
        round_continents.shuffle(rng);
        bonus_ufo_prompts.push(TimedPhasePrompt::SwapUFOLocations(
            round_continents[0].clone(),
            round_continents[1].clone(),
        ));
    }
    if round >= 4 {
        bonus_ufo_prompts.push(TimedPhasePrompt::AddUFOsToLocation(
            random_continent(rng),
            1,
        ));
    }
    if round >= 5 {
        bonus_ufo_prompts.push(TimedPhasePrompt::AddUFOsToLocation(
            random_continent(rng),
            1,
        ));
    }
    if round >= 6 {
        round_continents.shuffle(rng);
        bonus_ufo_prompts.push(TimedPhasePrompt::SwapUFOLocations(
            round_continents[0].clone(),
            round_continents[1].clone(),
        ));
    }
    if round >= 7 {
        bonus_ufo_prompts.push(TimedPhasePrompt::AddUFOsToLocation(
            random_continent(rng),
            2,
        ));
    }
    if round >= 8 {
        round_continents.shuffle(rng);
        bonus_ufo_prompts.push(TimedPhasePrompt::SwapUFOLocations(
            round_continents[0].clone(),
            round_continents[1].clone(),
        ));
    }

    if discover_alien_base {
        bonus_ufo_prompts.push(TimedPhasePrompt::AlienBaseDiscovered(random_continent(rng)));
    }

    let mut assign_interceptor_prompts: Vec<TimedPhasePrompt> = ALL_CONTINENTS
        .iter()
        .map(|continent| TimedPhasePrompt::AssignInterceptors(continent.clone()))
        .collect();

    let mut prompts = Vec::new();
    prompts.push(income_prompt);
    prompts.append(&mut ufo_prompts);
    prompts.append(&mut bonus_ufo_prompts);
    prompts.append(&mut assign_interceptor_prompts);
    prompts.push(TimedPhasePrompt::ChooseResearch);

    let num_shifts = match leftover_ufos {
        n if n < 1 => 0,
        n if n < 2 => 3,

        n if n < 4 => 5,
        n if n < 6 => 7,
        n if n < 9 => 9,
        n if n < 13 => 12,
        _ => 15,
    };
    for _ in 0..num_shifts {
        let from_pos = Uniform::new(0, prompts.len()).sample(rng);
        let removed_prompt = prompts.remove(from_pos);
        // We find the max by finding the first prompt which must come after the selected prompt
        let max_insert_pos = prompts
            .iter()
            .position(|other| other.must_come_after(&removed_prompt))
            .unwrap_or(prompts.len());
        // We find the min by finding the last prompt which must come before the selected prompt
        // We do this by first reversing the iterator, then finding the inverse position of any matching
        //   precursor prompt in the reversed iterator
        let min_insert_pos = prompts
            .iter()
            .rev()
            .position(|other| removed_prompt.must_come_after(other))
            .map(|pos| prompts.len() - pos)
            .unwrap_or(0);
        (min_insert_pos, max_insert_pos);
        if min_insert_pos < (max_insert_pos + 1) {
            let insert_pos = Uniform::new(min_insert_pos, max_insert_pos + 1).sample(rng);
            prompts.insert(insert_pos, removed_prompt)
        }
    }
    return prompts;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionPhasePrompt {
    AuditSpending,
    ResolveResearch,
    ResolveUFODefence,
    IncreasePanic,
    AskForBoardState,
    ResolveContinentBonuses,
    CleanUp,
    PurchaseReplacementForces,
}

impl ResolutionPhasePrompt {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AuditSpending,
            Self::ResolveResearch,
            Self::ResolveUFODefence,
            Self::IncreasePanic,
            Self::AskForBoardState,
            Self::ResolveContinentBonuses,
            Self::CleanUp,
            Self::PurchaseReplacementForces,
        ]
    }

    pub fn title(&self) -> String {
        match self {
            Self::AuditSpending => "Audit Spending",
            Self::ResolveResearch => "Resolve Research",
            Self::ResolveUFODefence => "Resolve UFO Defence",
            Self::IncreasePanic => "Increase Panic",
            Self::AskForBoardState => "Update Board State",
            Self::ResolveContinentBonuses => "Gain Continent Bonuses",
            Self::CleanUp => "Clean Up",
            Self::PurchaseReplacementForces => "Replenish Forces",
        }
        .to_owned()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameResult {
    Victory,
    PyrrhicVictory,
    Defeat,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Victory => write!(f, "Victory"),
            Self::PyrrhicVictory => write!(f, "Pyrrhic Victory"),
            Self::Defeat => write!(f, "Defeat"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case((TimedPhasePrompt::AssignInterceptors(Continent::Africa), TimedPhasePrompt::RollUFOLocation(Continent::Africa)),  true)]
    #[test_case((TimedPhasePrompt::AssignInterceptors(Continent::Africa), TimedPhasePrompt::RollUFOLocation(Continent::Eurasia)),  false)]
    #[test_case((TimedPhasePrompt::AddUFOsToLocation(Continent::Africa, 2), TimedPhasePrompt::RollUFOLocation(Continent::Africa)),  true)]
    #[test_case((TimedPhasePrompt::AddUFOsToLocation(Continent::Africa, 2), TimedPhasePrompt::RollUFOLocation(Continent::Eurasia)),  false)]
    #[test_case((TimedPhasePrompt::SwapUFOLocations(Continent::Africa, Continent::Eurasia), TimedPhasePrompt::RollUFOLocation(Continent::Eurasia)),  true)]
    #[test_case((TimedPhasePrompt::SwapUFOLocations(Continent::Africa, Continent::Eurasia), TimedPhasePrompt::RollUFOLocation(Continent::Africa)),  true)]
    #[test_case((TimedPhasePrompt::SwapUFOLocations(Continent::Africa, Continent::America), TimedPhasePrompt::RollUFOLocation(Continent::Eurasia)),  false)]
    fn succession_rules_test(prompts: (TimedPhasePrompt, TimedPhasePrompt), expected: bool) {
        assert_eq!(expected, prompts.0.must_come_after(&prompts.1));
    }
}
