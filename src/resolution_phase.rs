use xcom_1_card::{GameResult, PanicLevel, ResolutionPhasePrompt};
use yew::{
    html,
    web_sys::{Element, HtmlInputElement},
    Callback, ChangeData, Component, ComponentLink, Html, NodeRef, Properties,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanicLevelInput {
    PanicLevel(PanicLevel),
    AlienSpace,
}

impl Into<String> for PanicLevelInput {
    fn into(self) -> String {
        match self {
            PanicLevelInput::PanicLevel(PanicLevel::Yellow) => "yellow",
            PanicLevelInput::PanicLevel(PanicLevel::Orange) => "orange",
            PanicLevelInput::PanicLevel(PanicLevel::Red) => "red",
            PanicLevelInput::AlienSpace => "alien",
        }
        .to_owned()
    }
}

pub struct InvalidPanicLevelString;

impl TryFrom<&str> for PanicLevelInput {
    type Error = InvalidPanicLevelString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "yellow" => Ok(PanicLevelInput::PanicLevel(PanicLevel::Yellow)),
            "orange" => Ok(PanicLevelInput::PanicLevel(PanicLevel::Orange)),
            "red" => Ok(PanicLevelInput::PanicLevel(PanicLevel::Red)),
            "alien" => Ok(PanicLevelInput::AlienSpace),
            _ => Err(InvalidPanicLevelString),
        }
    }
}

pub struct ResolutionPhase {
    prompt: ResolutionPhasePrompt,
    alien_base_discovered: bool,
    panic_level_input: PanicLevelInput,
    ufos_left_input: u32,
    alien_base_destroyed_input: bool,
    link: ComponentLink<Self>,
    alien_base_destroyed_checkbox_ref: NodeRef,
    prompt_details_ref: NodeRef,
    on_completed: Callback<(PanicLevel, u32)>,
    on_game_end: Callback<GameResult>,
}

pub enum Msg {
    NextPrompt,
    PreviousPrompt,
    UpdatePanicLevel(PanicLevelInput),
    IncreaseUFOsLeft,
    DecreaseUFOsLeft,
    UpdateAlienBaseDestroyed(bool),
    CheckGameEnd,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub panic_level: PanicLevel,
    pub ufos_left: u32,
    pub alien_base_discovered: bool,
    pub on_completed: Callback<(PanicLevel, u32)>,
    pub on_game_end: Callback<GameResult>,
    #[prop_or_else(ResolutionPhasePrompt::start)]
    pub starting_prompt: ResolutionPhasePrompt,
}

impl Component for ResolutionPhase {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            prompt: props.starting_prompt,
            alien_base_discovered: props.alien_base_discovered,
            panic_level_input: PanicLevelInput::PanicLevel(props.panic_level.clone()),
            ufos_left_input: props.ufos_left,
            alien_base_destroyed_input: false,
            alien_base_destroyed_checkbox_ref: NodeRef::default(),
            prompt_details_ref: NodeRef::default(),
            link,
            on_completed: props.on_completed,
            on_game_end: props.on_game_end,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::NextPrompt => {
                if let Some(next_prompt) = self.prompt.next() {
                    self.prompt = next_prompt;
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    if let PanicLevelInput::PanicLevel(level) = self.panic_level_input.clone() {
                        self.on_completed.emit((level, self.ufos_left_input));
                        false
                    } else {
                        false
                    }
                }
            }
            Msg::PreviousPrompt => {
                if let Some(prev_prompt) = self.prompt.prev() {
                    self.prompt = prev_prompt;
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::UpdatePanicLevel(panic_level_input) => {
                self.panic_level_input = panic_level_input;
                false
            }
            Msg::IncreaseUFOsLeft => {
                if self.ufos_left_input < 18 {
                    self.ufos_left_input += 1;
                }
                true
            }
            Msg::DecreaseUFOsLeft => {
                if self.ufos_left_input > 0 {
                    self.ufos_left_input -= 1;
                }
                true
            }
            Msg::UpdateAlienBaseDestroyed(alien_base_destroyed) => {
                self.alien_base_destroyed_input = alien_base_destroyed;
                false
            }
            Msg::CheckGameEnd => {
                match (
                    self.alien_base_destroyed_input,
                    self.panic_level_input.clone(),
                ) {
                    (true, PanicLevelInput::PanicLevel(_)) => {
                        self.on_game_end.emit(GameResult::Victory)
                    }
                    (true, PanicLevelInput::AlienSpace) => {
                        self.on_game_end.emit(GameResult::PyrrhicVictory)
                    }
                    (false, PanicLevelInput::AlienSpace) => {
                        self.on_game_end.emit(GameResult::Defeat)
                    }
                    (false, PanicLevelInput::PanicLevel(_)) => (),
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.prompt = props.starting_prompt;
        self.alien_base_discovered = props.alien_base_discovered;
        self.panic_level_input = PanicLevelInput::PanicLevel(props.panic_level.clone());
        self.ufos_left_input = props.ufos_left;
        self.on_completed = props.on_completed;
        self.on_game_end = props.on_game_end;
        true
    }

    fn view(&self) -> yew::Html {
        let checkbox_ref = self.alien_base_destroyed_checkbox_ref.clone();
        let panic_input_change_callback = self.link.batch_callback(|c: ChangeData| {
            if let ChangeData::Value(val) = c {
                if let Ok(panic_level_input) = PanicLevelInput::try_from(val.as_str()) {
                    return vec![Msg::UpdatePanicLevel(panic_level_input)];
                }
            }
            return vec![];
        });
        let main_section = match self.prompt {
            ResolutionPhasePrompt::AskForBoardState => {
                let current_panic_level: String = self.panic_level_input.clone().into();
                html! {
                    <div class="board-input-container">
                        <div>
                            <div class="board-input-title">{ "Global panic level:"} </div>
                            <div class="panic-input-container">
                            {
                                vec!["yellow", "orange", "red", "alien"].into_iter()
                                    .map(|input| html!{
                                        <>
                                            <input
                                                class="panic-input-radio"
                                                type="radio"
                                                id=format!("panic-{}", input)
                                                name="panic-input"
                                                value=input
                                                onchange=panic_input_change_callback.clone()
                                                checked=input == current_panic_level
                                            />
                                            <label
                                                class="panic-input-label"
                                                for=format!("panic-{}", input)
                                            >
                                                <img src=format!("assets/icons/panic-input-{}.png", input)/>
                                            </label>
                                        </>
                                    })
                                    .collect::<Html>()
                            }
                            </div>
                        </div>
                        <div>
                            <div class="board-input-title">{ "UFOs left on map:"} </div>
                            <div class="ufo-input-container">
                                <button class="ufo-input-button" onclick=self.link.callback(|_| Msg::DecreaseUFOsLeft)>{"-"}</button>
                                <span class="ufo-input-text" >{ self.ufos_left_input }</span>
                                <button class="ufo-input-button" onclick=self.link.callback(|_| Msg::IncreaseUFOsLeft)>{"+"}</button>
                            </div>
                        </div>
                        {
                            if self.alien_base_discovered {
                            html!{
                                <div class="alien-base-destroyed-input-container">
                                    <label for="alien_base_destroyed_input">{ "Alien base destroyed?" }</label>
                                    <input
                                    class="alien-base-destroyed-input-checkbox"
                                        type="checkbox"
                                        name="alien_base_destroyed_input"
                                        ref=self.alien_base_destroyed_checkbox_ref.clone()
                                        checked=self.alien_base_destroyed_input
                                        onchange=self.link.batch_callback(move |_: ChangeData| {
                                            if let Some(input_element) = checkbox_ref.cast::<HtmlInputElement>() {
                                                return vec![Msg::UpdateAlienBaseDestroyed(input_element.checked())];
                                            }
                                            return vec![];
                                        })
                                    />
                                </div>
                                }
                            } else {
                                html!{}
                            }
                        }
                    </div>
                }
            }
            _ => html! {
                <>
                    <h1 class="prompt-title">{ self.prompt.title() }</h1>
                    <div class="prompt-center-area">
                        <div class="side-buttons">
                        </div>
                        <div class="prompt-details" ref=self.prompt_details_ref.clone()>
                            <div class="prompt-icons">
                                {icon_html_for_prompt(&self.prompt)}
                            </div>
                            <div class="prompt-description">
                                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                            </div>
                        </div>
                    </div>
                </>
            },
        };
        let next_callback = match self.prompt {
            ResolutionPhasePrompt::AskForBoardState => self
                .link
                .batch_callback(|_| vec![Msg::CheckGameEnd, Msg::NextPrompt]),
            _ => self.link.callback(|_| Msg::NextPrompt),
        };
        html! {
            <>
                {main_section}
                <div class="bottom-panel">
                    <button class="button-back" onclick=self.link.callback(|_| Msg::PreviousPrompt)>{ "Back" }</button>
                    <button class="button-done" onclick=next_callback>{ "Done" }</button>
                </div>
            </>
        }
    }
}

fn icon_html_for_prompt(prompt: &ResolutionPhasePrompt) -> Html {
    match prompt {
        ResolutionPhasePrompt::AuditSpending => html! {
            <img class="prompt-icon" src="assets/icons/audit.png"/>
        },
        ResolutionPhasePrompt::ResolveResearch => html! {
            <img class="prompt-icon" src="assets/icons/research.png"/>
        },
        ResolutionPhasePrompt::ResolveUFODefense => html! {
            <img class="prompt-icon" src="assets/icons/interceptor.png"/>
        },
        ResolutionPhasePrompt::IncreasePanic => html! {
            <img class="prompt-icon" src="assets/icons/alien.png"/>
        },
        ResolutionPhasePrompt::AskForBoardState => html! {},
        ResolutionPhasePrompt::ResolveContinentBonuses => html! {
            <>
                <img class="prompt-icon" src="assets/icons/america.png"/>
                <img class="prompt-icon" src="assets/icons/africa.png"/>
                <img class="prompt-icon" src="assets/icons/eurasia.png"/>
            </>
        },
        ResolutionPhasePrompt::CleanUp => html! {
            <img class="prompt-icon" src="assets/icons/cleanup.png"/>
        },
        ResolutionPhasePrompt::PurchaseReplacementForces => html! {
            <img class="prompt-icon" src="assets/icons/replenish.png"/>
        },
    }
}
