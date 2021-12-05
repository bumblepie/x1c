use rand::{
    distributions::{Uniform, WeightedIndex},
    prelude::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanicLevel {
    Yellow,
    Orange,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Continent {
    America,
    Africa,
    Eurasia,
}

pub const ALL_CONTINENTS: [Continent; 3] =
    [Continent::America, Continent::Africa, Continent::Eurasia];

pub fn random_continent<R>(rng: &mut R) -> Continent
where
    R: Rng,
{
    ALL_CONTINENTS.choose(rng).unwrap().clone()
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionPhasePrompt {
    AuditSpending,
    ResolveResearch,
    ResolveUFODefense,
    IncreasePanic,
    AskForBoardState,
    ResolveContinentBonuses,
    CleanUp,
    PurchaseReplacementForces,
}

impl ResolutionPhasePrompt {
    pub fn start() -> Self {
        Self::AuditSpending
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Self::AuditSpending => Some(Self::ResolveResearch),
            Self::ResolveResearch => Some(Self::ResolveUFODefense),
            Self::ResolveUFODefense => Some(Self::IncreasePanic),
            Self::IncreasePanic => Some(Self::AskForBoardState),
            Self::AskForBoardState => Some(Self::ResolveContinentBonuses),
            Self::ResolveContinentBonuses => Some(Self::CleanUp),
            Self::CleanUp => Some(Self::PurchaseReplacementForces),
            Self::PurchaseReplacementForces => None,
        }
    }

    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::AuditSpending => None,
            Self::ResolveResearch => Some(Self::AuditSpending),
            Self::ResolveUFODefense => Some(Self::ResolveResearch),
            Self::IncreasePanic => Some(Self::ResolveUFODefense),
            Self::AskForBoardState => Some(Self::IncreasePanic),
            Self::ResolveContinentBonuses => Some(Self::AskForBoardState),
            Self::CleanUp => Some(Self::ResolveContinentBonuses),
            Self::PurchaseReplacementForces => Some(Self::CleanUp),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameResult {
    Victory,
    PyrrhicVictory,
    Defeat,
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
