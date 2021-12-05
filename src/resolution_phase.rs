use xcom_1_card::{GameResult, PanicLevel, ResolutionPhasePrompt};
use yew::{
    html, web_sys::HtmlInputElement, Callback, ChangeData, Component, ComponentLink, NodeRef,
    Properties,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanicLevelInput {
    PanicLevel(PanicLevel),
    AlienSpace,
}

impl Into<i32> for PanicLevelInput {
    fn into(self) -> i32 {
        match self {
            PanicLevelInput::PanicLevel(PanicLevel::Yellow) => 0,
            PanicLevelInput::PanicLevel(PanicLevel::Orange) => 1,
            PanicLevelInput::PanicLevel(PanicLevel::Red) => 2,
            PanicLevelInput::AlienSpace => 3,
        }
    }
}

pub struct InvalidPanicLevelNum;

impl TryFrom<i32> for PanicLevelInput {
    type Error = InvalidPanicLevelNum;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PanicLevelInput::PanicLevel(PanicLevel::Yellow)),
            1 => Ok(PanicLevelInput::PanicLevel(PanicLevel::Orange)),
            2 => Ok(PanicLevelInput::PanicLevel(PanicLevel::Red)),
            3 => Ok(PanicLevelInput::AlienSpace),
            _ => Err(InvalidPanicLevelNum),
        }
    }
}

pub struct ResolutionPhase {
    prompt: ResolutionPhasePrompt,
    panic_level_input: PanicLevelInput,
    ufos_left_input: u32,
    alien_base_destroyed_input: bool,
    link: ComponentLink<Self>,
    alien_base_destroyed_checkbox: NodeRef,
    props: Props,
}

pub enum Msg {
    NextPrompt,
    PreviousPrompt,
    UpdatePanicLevel(PanicLevelInput),
    UpdateUfosLeft(u32),
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
}

impl Component for ResolutionPhase {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            prompt: ResolutionPhasePrompt::start(),
            panic_level_input: PanicLevelInput::PanicLevel(props.panic_level.clone()),
            ufos_left_input: props.ufos_left,
            alien_base_destroyed_input: false,
            alien_base_destroyed_checkbox: NodeRef::default(),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::NextPrompt => {
                if let Some(next_prompt) = self.prompt.next() {
                    self.prompt = next_prompt;
                    true
                } else {
                    if let PanicLevelInput::PanicLevel(level) = self.panic_level_input.clone() {
                        self.props.on_completed.emit((level, self.ufos_left_input));
                        false
                    } else {
                        false
                    }
                }
            }
            Msg::PreviousPrompt => {
                if let Some(prev_prompt) = self.prompt.prev() {
                    self.prompt = prev_prompt;
                    true
                } else {
                    false
                }
            }
            Msg::UpdatePanicLevel(panic_level_input) => {
                self.panic_level_input = panic_level_input;
                false
            }
            Msg::UpdateUfosLeft(ufos_left) => {
                self.ufos_left_input = ufos_left;
                false
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
                        self.props.on_game_end.emit(GameResult::Victory)
                    }
                    (true, PanicLevelInput::AlienSpace) => {
                        self.props.on_game_end.emit(GameResult::PyrrhicVictory)
                    }
                    (false, PanicLevelInput::AlienSpace) => {
                        self.props.on_game_end.emit(GameResult::Defeat)
                    }
                    (false, PanicLevelInput::PanicLevel(_)) => (),
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> yew::Html {
        let checkbox_ref = self.alien_base_destroyed_checkbox.clone();
        let main = match self.prompt {
            ResolutionPhasePrompt::AskForBoardState => {
                let panic_level_num: i32 = self.panic_level_input.clone().into();
                html! {
                    <>
                        <div>
                            <p>{ "Global panic level:"} </p>
                            <input
                                type="range"
                                min="0" max="3" step="1"
                                value=format!("{}", panic_level_num)
                                onchange=self.link.batch_callback(|c: ChangeData| {
                                    if let ChangeData::Value(val) = c {
                                        if let Ok(val) = val.parse::<i32>() {
                                            if let Ok(panic_level_input) = PanicLevelInput::try_from(val) {
                                                return vec![Msg::UpdatePanicLevel(panic_level_input)]
                                            }
                                        }
                                    }
                                    return vec![];
                                })
                            />
                        </div>
                        <div>
                            <p>{ "UFOs left on map:"} </p>
                            <input
                                type="number"
                                min="0" max="18" step="1"
                                value=format!("{}", self.ufos_left_input)
                                onchange=self.link.batch_callback(|c: ChangeData| {
                                    if let ChangeData::Value(val) = c {
                                        if let Ok(val) = val.parse::<u32>() {
                                            return vec![Msg::UpdateUfosLeft(val)]
                                        }
                                    }
                                    return vec![];
                                })
                            />
                        </div>
                        {
                            if self.props.alien_base_discovered {
                            html!{
                                <div>
                                    <input
                                        type="checkbox"
                                        name="alien_base_destroyed_input"
                                        ref=self.alien_base_destroyed_checkbox.clone()
                                        checked=self.alien_base_destroyed_input
                                        onchange=self.link.batch_callback(move |_: ChangeData| {
                                            if let Some(input_element) = checkbox_ref.cast::<HtmlInputElement>() {
                                                return vec![Msg::UpdateAlienBaseDestroyed(input_element.checked())];
                                            }
                                            return vec![];
                                        })
                                    />
                                    <label for="alien_base_destroyed_input">{ "Alien base destroyed?" }</label>
                                </div>
                                }
                            } else {
                                html!{}
                            }
                        }
                    </>
                }
            }
            _ => html! {},
        };
        let next_callback = match self.prompt {
            ResolutionPhasePrompt::AskForBoardState => self
                .link
                .batch_callback(|_| vec![Msg::CheckGameEnd, Msg::NextPrompt]),
            _ => self.link.callback(|_| Msg::NextPrompt),
        };
        html! {
            <>
                <div>
                    <p>{ format!("{:?}", self.prompt) }</p>
                    { main }
                </div>
                <div>
                    <button onclick=self.link.callback(|_| Msg::PreviousPrompt)>{ "Prev" }</button>
                    <button onclick=next_callback>{ "Next" }</button>
                </div>
            </>
        }
    }
}
