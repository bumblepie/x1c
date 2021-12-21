use yew::{
    html, web_sys::Element, Callback, Component, ComponentLink, MouseEvent, NodeRef, Properties,
};

use crate::common::inline_icon_text_phrase;

pub enum SetupStep {
    ListComponents,
    Theme,
    WinLossConditions,
    GameStructure,
    Panic,
    Funding,
    WorldMap,
    UFOEdgeCases,
    UFOs,
    AlienThreatAndTasks,
    AlienThreatConsequences,
    Interceptors,
    UFODefense,
    ResearchAndTech,
    ResearchTask,
    UsingTech,
    Satellites,
}

impl SetupStep {
    pub fn get_all_steps() -> Vec<SetupStep> {
        vec![
            Self::ListComponents,
            Self::Theme,
            Self::WinLossConditions,
            Self::GameStructure,
            Self::Panic,
            Self::Funding,
            Self::WorldMap,
            Self::UFOs,
            Self::UFOEdgeCases,
            Self::AlienThreatAndTasks,
            Self::AlienThreatConsequences,
            Self::Interceptors,
            Self::UFODefense,
            Self::ResearchAndTech,
            Self::ResearchTask,
            Self::UsingTech,
            Self::Satellites,
        ]
    }
}

pub enum Msg {
    NextPrompt,
    PrevPrompt,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub on_completed: Callback<MouseEvent>,
}

pub struct SetupComponent {
    steps: Vec<SetupStep>,
    current_step_index: usize,
    prompt_details_ref: NodeRef,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for SetupComponent {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            steps: SetupStep::get_all_steps(),
            current_step_index: 0,
            prompt_details_ref: NodeRef::default(),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::NextPrompt => {
                if self.current_step_index < self.steps.len() {
                    self.current_step_index += 1;
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::PrevPrompt => {
                if self.current_step_index > 0 {
                    self.current_step_index -= 1;
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> yew::Html {
        if self.current_step_index == self.steps.len() {
            html! {
                <>
                    <h1 class="prompt-title">{"Training complete"}</h1>
                    <div class="prompt-center-area">
                        <div class="side-buttons">
                        </div>
                        <p class="prompt-description">{"Your training is complete Commander, the real battle begins now. Good luck."}</p>
                    </div>
                    <div class="bottom-panel">
                        <button class="button-back" onclick=self.link.callback(|_| Msg::PrevPrompt) disabled={ self.current_step_index < 1 }>{ "Back" }</button>
                        <button class="button-done" onclick=self.props.on_completed.clone()>{ "Done" }</button>
                    </div>
                </>
            }
        } else {
            let (title, main) = match self.steps[self.current_step_index] {
                SetupStep::ListComponents => (
                    "Components",
                    html! {
                        <div class="components-grid">
                            <div class="components">
                                <h3>{"From XCOM: TBG"}</h3>
                                <ul>
                                    <li>{"4 Success Dice (or BYO 4 D6s)"}</li>
                                    <li>{"1 Alien Die (or BYO 1 D8)"}</li>
                                    <li>{"20 Credit tokens (or BYO money tokens)"}</li>
                                    <li>{"6 Success tokens (or BYO tokens)"}</li>
                                </ul>
                            </div>
                            <div class="components">
                                <h3>{"Print n Play"}</h3>
                                <ul>
                                    <li>{"1 Main card"}</li>
                                    <li>{"9 Technology tokens"}</li>
                                    <li>{"1 Alien Base token"}</li>
                                </ul>
                            </div>
                            <div class="components">
                                <h3>{"Cubes and Dice"}</h3>
                                <ul>
                                    <li>
                                        {"16 8mm cubes"}
                                        <ul>
                                            <li>{"12 Interceptor cubes"}</li>
                                            <li>{"1 Panic marker"}</li>
                                            <li>{"1 Research Budget marker"}</li>
                                            <li>{"1 Satellites marker"}</li>
                                            <li>{"1 Alien Threat marker"}</li>
                                        </ul>
                                    </li>
                                    <li>{"3 UFO Dice (D6s)"}</li>
                                </ul>
                            </div>
                        </div>
                    },
                ),
                SetupStep::Theme => (
                    "Welcome, Commander",
                    html! {
                        <>
                            <p>
                                {"Welcome, Commander. You have been chosen to lead X-1C, the last line of defense against the alien invasion. Your job is to decide where to allocate our limited resources to best defend ourselves."}
                            </p>
                            <p>
                                {"To prevent the world falling into panic, we'll need to deploy our interceptors to bring down UFOs in different parts of the world. We'll also need to research the alien technology to better equip ourselves if we're to stand any chance in the long run."}
                            </p>
                            <p>
                                {"Our top data scientists are working to discover where the aliens have set up their base - if we can survive long enough, we can find out where it is and launch an attack against it. If we can destroy the base, it'll turn the tide and allow us to win the war."}
                            </p>
                            <p>
                                {"Good luck Commander, the future of humanity is in your hands."}
                            </p>
                        </>
                    },
                ),
                SetupStep::WinLossConditions => (
                    "Goal of the Game",
                    html! {
                        <>
                            <p>
                                {"Your goal in this game is to find and destroy the "}{inline_icon_text_phrase("alien-base", "Alien Base")}{" without letting the world fall into panic."}
                            </p>
                            <p>
                                {"To destroy the "}{inline_icon_text_phrase("alien-base", "Alien Base,")}{" you'll need to survive for several rounds. You will eventually be prompted to place the "}{inline_icon_text_phrase("alien-base", "Alien Base Token")}{" in a certain part of the world - once it's been discovered you'll be able to attempt to destroy the base."}
                            </p>
                            <p>
                                {"If the "}{inline_icon_text_phrase("panic", "Global Panic Level")}{" ever reaches the last space on its track, the world falls into panic and you lose the game."}
                            </p>
                        </>
                    },
                ),
                SetupStep::GameStructure => (
                    "Game Structure",
                    html! {
                        <>
                            <p>
                                {"The game is played over several rounds. Each round consists of a "}{inline_icon_text_phrase("time", "Timed Phase")}{" in which you decide where to allocate your resources in a limited amount of time, followed by a "}{inline_icon_text_phrase("resolution", "Resolution Phase")}{" in which you resolve the consequences of your decisions."}
                            </p>
                            <p>
                                {"During the "}{inline_icon_text_phrase("time", "Timed Phase,")}{" you will need to follow the directions of the app to place and move UFOs on the world map, and make decisions about how to use your resources. Once finished with each prompt, click the \"DONE\" button to move to the next prompt."}
                            </p>
                        </>
                    },
                ),
                SetupStep::Panic => (
                    "Panic",
                    html! {
                        <>
                            <p>
                                <b>{"Place the "}{inline_icon_text_phrase("panic", "Panic marker")}{" on the first space of the "}{inline_icon_text_phrase("panic", "Panic Track")}{":"}</b>
                            </p>
                            <p>
                                {"This represents the current "}{inline_icon_text_phrase("panic", "Global Panic Level.")}
                            </p>
                            <p>
                                {"Leaving UFOs unchecked and failing to pay for your resources will cause the "}{inline_icon_text_phrase("panic", "Global Panic Level")}{" to increase, moving the marker up the track. As the marker moves up the track, the world becomes more disorganised, reducing the funding available to you. If it ever reaches the final space, the world falls into chaos and you lose the game."}
                            </p>
                        </>
                    },
                ),
                SetupStep::Funding => (
                    "Funding",
                    html! {
                        <>
                            <p>
                                <b>{"Set aside 5 credit tokens to form your funds."}</b>
                            </p>
                            <p>
                                {"Each round during the "}{inline_icon_text_phrase("time", "Timed Phase,")}{" you will gain additional funds."}
                            </p>
                            <p>
                                {"When you deploy "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" and put points in your "}{inline_icon_text_phrase("research", "Research Budget")}{" during the "}{inline_icon_text_phrase("time", "Timed Phase,")}{" it will cost you credits during the "}{inline_icon_text_phrase("resolution", "Resolution Phase.")}{" If you are unable to pay, the "}{inline_icon_text_phrase("panic", "Global Panic Level")}{" will increase, so be careful not to overspend."}
                            </p>
                            <p>
                                {"Whenever you see §X, that means X credit tokens."}
                            </p>
                        </>
                    },
                ),
                SetupStep::WorldMap => (
                    "The World Map",
                    html! {
                        <>
                            <p>
                                {"In the center of the main card, you will find the World Map. This will be the theatre in which humanity's struggle for survival will be played out:"}
                            </p>
                            <p>
                                {"There are three continents of interest: America, Africa and Eurasia. Each of these has a square in which a UFO die can be placed, and a set of four smaller squares to which your "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" can be deployed."}
                            </p>
                        </>
                    },
                ),
                SetupStep::UFOs => (
                    "UFO Invasion",
                    html! {
                        <>
                            <p>
                                {"During the "}{inline_icon_text_phrase("time", "Timed Phase,")}{" UFOs will descend from orbit and move around the world map. To signify how many UFOs are over each continent, we use a UFO die (a D6) in the appropriate space."}
                            </p>
                            <p>
                                {"You will usually roll the die for a continent to determine how many UFOs initially approach the continent."}
                            </p>
                            <p>
                                {"You may also be asked to increase the number of UFOs in a continent. To do this, turn the die to show the new number of UFOs. Similarly, when your "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" shoot down UFOs, turn the die to show the number of remaining UFOs."}
                            </p>
                        </>
                    },
                ),
                SetupStep::UFOEdgeCases => (
                    "UFO Special Cases",
                    html! {
                        <>
                            <h3 class="prompt-title">
                                {"Note some special situations:"}
                            </h3>
                            <p>
                                {"A continent cannot have more than 6 UFOs. If you are asked to increase the number of UFOs in a continent beyond 6, any additional UFOs are added to any other continent of your choice instead. If all three continents contain 6 UFOs, no additional UFOs are added."}
                            </p>
                            <p>
                                {"When asked to roll the UFO die for a continent which contains UFOs, move as many of those UFOs  as possible to the other continents before rolling the die. If both other continents are already full or are filled by doing this, ignore any remaining UFOs which cannot be moved - do not re-add them to the continent after rolling the die."}
                            </p>
                            <p>
                                {"When asked to increase the number of UFOs for a continent which contains no UFOs, just add a UFO die to the space set to the number of UFOs requested."}
                            </p>
                        </>
                    },
                ),
                SetupStep::AlienThreatAndTasks => (
                    "Alien Threat and Tasks",
                    html! {
                        <>
                            <p>
                                <b>{"Place the "}{inline_icon_text_phrase("alien", "Alien Threat marker")}{" on the first space of the "}{inline_icon_text_phrase("alien", "Alien Threat Track:")}</b>
                            </p>
                            <p>
                                {"This is used to track the increasing danger as you attempt and re-attempt tasks."}
                            </p>
                            <p>
                                {"During the "}{inline_icon_text_phrase("resolution", "Resolution Phase,")}{" you will be asked to resolve tasks such as "}{inline_icon_text_phrase("research", "Research")}{" and "}{inline_icon_text_phrase("interceptor", "UFO defense.")}{" These use the same core mechanics:"}
                            </p>
                            <p>
                                {"Based on the resources allocated to the task, you will roll a number of "}{inline_icon_text_phrase("success", "Success Dice")}{" as well as the "}{inline_icon_text_phrase("alien", "Alien Die.")}
                            </p>
                            <p>
                                {"Any "}{inline_icon_text_phrase("success", "Successes")}{" rolled (the "}{inline_icon_text_phrase("success", "Success Icon")}{" on the XCOM dice or a 5+ if using D6s) will have a positive effect, such as progress towards researching "}{inline_icon_text_phrase("tech", "Technology.")}
                            </p>
                        </>
                    },
                ),
                SetupStep::AlienThreatConsequences => (
                    "Alien Threat and Tasks",
                    html! {
                        <>
                            <p>
                                {"Be careful though: if the value you roll on the "}{inline_icon_text_phrase("alien", "Alien Die")}{" is less than or equal to the current "}{inline_icon_text_phrase("alien", "Alien Threat Level,")}{" you face severe negative consequences such as losing "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" in combat."}
                            </p>
                            <p>
                                {"After resolving each roll, you may choose to try to get more "}{inline_icon_text_phrase("success", "Successes")}{" by rolling both the "}{inline_icon_text_phrase("success", "Success Dice")}{" and the "}{inline_icon_text_phrase("alien", "Alien Die")}{" again. However, the risk involved increases: after each attempt, increase the "}{inline_icon_text_phrase("alien", "Alien Threat Level")}{" one space along the track (to a maximum of 5)."}
                            </p>
                            <p>
                                {"If this seems too risky, you may choose to stop rolling instead, but once you have stopped rolling against a task you may not attempt it again this round."}
                            </p>
                            <p>
                                {"When you begin a new task, the "}{inline_icon_text_phrase("alien", "Alien Threat Level")}{" is reset to the first space on the track."}
                            </p>
                        </>
                    },
                ),
                SetupStep::Interceptors => (
                    "Interceptors",
                    html! {
                        <>
                            <p>
                                <b>
                                    {"Set aside 8 "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" to form your reserves:"}
                                </b>
                            </p>
                            <p>
                                {"Your reserves contain your "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" that are ready to be deployed - as the game goes on, you might lose "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" that are shot down by UFOs. If this happens, you'll need to pay credits (§1 each) to build replacements."}
                            </p>
                            <p>
                                {"As well as a space for the UFO dice, the continents have spaces to deploy "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" to combat the UFOs:"}
                            </p>
                            <p>
                                {"You choose how many "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" to deploy to each continent during the "}{inline_icon_text_phrase("time", "Timed Phase.")}{" Deploying "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" will cost §1 each from your funds."}
                            </p>
                        </>
                    },
                ),
                SetupStep::UFODefense => (
                    "UFO Defense",
                    html! {
                        <>
                            <p>
                                {"During the "}{inline_icon_text_phrase("resolution", "Resolution Phase,")}{" deployed "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" will have a chance to shoot down the UFOs via the UFO Defense task - the more interceptors assigned, the more "}{inline_icon_text_phrase("success", "Success Dice")}{" you roll each attempt."}
                            </p>
                            <p>
                                {"Any remaining UFOs in a continent will increase "}{inline_icon_text_phrase("panic", "Global Panic")}{" and scramble our communications, making the prompts in the next "}{inline_icon_text_phrase("time", "Timed Phase")}{" come out of order."}
                            </p>
                            <p>
                                {"The consequence of the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" during the "}{inline_icon_text_phrase("interceptor", "UFO Defense Task")}{" is that the UFOs destroy half of your "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" (rounded up), returning them to the supply rather than your reserves. You can continue rolling against the task, but you'll do so with less firepower and the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" will continue to increase."}
                            </p>
                        </>
                    },
                ),
                SetupStep::ResearchAndTech => (
                    "Research and Technology",
                    html! {
                        <>
                            <p>
                                <b>
                                    {"Place the "}{inline_icon_text_phrase("research", "Research Budget marker")}{" at the zero space at the bottom of the "}{inline_icon_text_phrase("research", "Research Budget Track:")}
                                </b>
                            </p>
                            <p>
                                {"This represents your investment into researching "}{inline_icon_text_phrase("tech", "Alien Technology.")}{" Each "}{inline_icon_text_phrase("tech", "Technology")}{" gives a special power that can be used once during each round for no cost."}
                            </p>
                            <p>
                                {"You choose which "}{inline_icon_text_phrase("tech", "Technology.")}{" to research, as well as how much to invest into "}{inline_icon_text_phrase("research", "Research")}{" during the "}{inline_icon_text_phrase("time", "Timed Phase.")}{" Each point in the "}{inline_icon_text_phrase("research", "Research Budget")}{" costs §1."}
                            </p>
                        </>
                    },
                ),
                SetupStep::ResearchTask => (
                    "Research",
                    html! {
                        <>
                            <p>
                                {"During the "}{inline_icon_text_phrase("resolution", "Resolution Phase,")}{" you will have a chance to gain the chosen "}{inline_icon_text_phrase("tech", "Alien Technology")}{" by completing the "}{inline_icon_text_phrase("research", "Research Task.")}
                            </p>
                            <p>
                                {" The higher the "}{inline_icon_text_phrase("research", "Research Budget,")}{" the more "}{inline_icon_text_phrase("success", "Success Dice")}{" you roll during this task. Gaining "}{inline_icon_text_phrase("success", "Successes")}{" equal to the "}{inline_icon_text_phrase("research", "Research Cost")}{" of the chosen "}{inline_icon_text_phrase("tech", "Technology")}{" will let you use that tech's power from then on."}
                            </p>
                            <p>
                                {"Be careful though, the consequence of the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" during the "}{inline_icon_text_phrase("research", "Research Task")}{" is that the volatile "}{inline_icon_text_phrase("tech", "Alien Technology")}{" explodes, removing it from the game permanently!"}
                            </p>
                            <p>
                                {"If you choose to stop rolling before completely researching a "}{inline_icon_text_phrase("tech", "Technology,")}{" any success tokens will stay on the chosen technology. If you choose to start researching a different "}{inline_icon_text_phrase("tech", "Technology")}{" however, discard any "}{inline_icon_text_phrase("success", "Success Tokens")}{" on the "}{inline_icon_text_phrase("tech", "Technology")}{" that you are no longer researching."}
                            </p>
                        </>
                    },
                ),
                SetupStep::UsingTech => (
                    "Using Technology",
                    html! {
                        <>
                            <p>
                                {"Each "}{inline_icon_text_phrase("tech", "Technology")}{" has a unique effect and can only be used once per round, either in the "}{inline_icon_text_phrase("time", "Timed Phase")}{" or in the "}{inline_icon_text_phrase("resolution", "Resolution Phase")}{" as indicated on its token."}
                            </p>
                            <p>
                                {"After using a "}{inline_icon_text_phrase("tech", "Technology")}{" you have researched, flip the "}{inline_icon_text_phrase("tech", "Technology token")}{" to the other side to show it has been used."}
                            </p>
                            <p>
                                {"At any time, you can see detailed information about each "}{inline_icon_text_phrase("tech", "Alien Technology")}{" by hitting the button on the left:"}
                            </p>
                        </>
                    },
                ),
                SetupStep::Satellites => (
                    "Satellites",
                    html! {
                        <>
                            <p>
                                <b>
                                    {"Place the "}{inline_icon_text_phrase("satellite", "Satellite marker")}{" at the three space at the top of the "}{inline_icon_text_phrase("satellite", "Satellite Track:")}
                                </b>
                            </p>
                            <p>
                                {"Your "}{inline_icon_text_phrase("satellite", "Satellites")}{" are powerful tools to help you manage your situation. At any point during the "}{inline_icon_text_phrase("time", "Timed Phase")}{"you can spend a "}{inline_icon_text_phrase("satellite", "Satellite,")}{" moving the "}{inline_icon_text_phrase("satellite", "Satellite marker")}{" down one space, to either:"}
                            </p>
                            <p class="prompt-success-outcome-container">
                                {"Reroll a UFO die immediately after seeing the result. You must use the new result."}
                            </p>
                            <p> {"or"}</p>
                            <div class="prompt-success-outcome-container">
                                <p>
                                    {"Move up to three "}{inline_icon_text_phrase("interceptor", "Interceptors.")}
                                </p>
                                <p>
                                    {"These may be deployed from your reserves to any continent, moved between any two continents, or returned from a continent back to your reserves."}
                                </p>
                            </div>
                            <p>
                                {"You may replenish your "}{inline_icon_text_phrase("satellite", "Satellites")}{" for §2 each when prompted during the "}{inline_icon_text_phrase("resolution", "Resolution Phase.")}
                            </p>
                        </>
                    },
                ),
            };
            html! {
                <>
                <h1 class="prompt-title">{title}</h1>
                <div class="prompt-center-area">
                    <div class="side-buttons">
                    </div>
                    <div class="prompt-details" ref=self.prompt_details_ref.clone()>
                        <div class="prompt-description">
                            {main}
                        </div>
                    </div>
                </div>
                <div class="bottom-panel">
                    <button class="button-back" onclick=self.link.callback(|_| Msg::PrevPrompt) disabled={ self.current_step_index < 1 }>{ "Back" }</button>
                    <button class="button-done" onclick=self.link.callback(|_| Msg::NextPrompt)>{ "Done" }</button>
                </div>
                </>
            }
        }
    }
}
