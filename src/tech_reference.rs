use std::fmt::Display;

use crate::common::inline_icon_text_phrase;
use boolinator::Boolinator;
use yew::prelude::*;

enum PhaseUsage {
    Timed,
    Resolution,
}

impl Display for PhaseUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timed => write!(f, "Timed"),
            Self::Resolution => write!(f, "Resolution"),
        }
    }
}

pub struct TechInfo {
    name: String,
    icon: String,
    cost: u32,
    phase: PhaseUsage,
    description: Html,
}

impl TechInfo {
    fn render_details(&self) -> Html {
        html! {
            <div>
                <div class="tech-title">
                    <h3>{&self.name}</h3>
                    <div>{format!("Phase: {}", self.phase)}</div>
                    <div>{format!("Research Cost: {}", self.cost)}</div>
                </div>
                <p class="tech-description">
                    {self.description.clone()}
                </p>
            </div>
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tech {
    UFOTracking,
    UFONavigation,
    HyperwaveCommunication,
    DefenceMatrix,
    EMPCannon,
    Firestorm,
    AlienIntel,
    CarapaceArmour,
    EleriumGenerator,
}

impl Tech {
    fn get_all() -> Vec<Tech> {
        vec![
            Self::UFOTracking,
            Self::UFONavigation,
            Self::HyperwaveCommunication,
            Self::DefenceMatrix,
            Self::EMPCannon,
            Self::Firestorm,
            Self::AlienIntel,
            Self::CarapaceArmour,
            Self::EleriumGenerator,
        ]
    }

    fn get_info(&self) -> TechInfo {
        match self {
            Self::HyperwaveCommunication => TechInfo {
                name: "Hyperwave Communication".to_owned(),
                icon: "assets/tech/hyperwave-communication.png".to_owned(),
                cost: 3,
                phase: PhaseUsage::Timed,
                description: html! {
                    <>
                    {"Immediately after rolling a UFO die, you may reroll it and take the new result instead. This may be used in conjuction with a "}{inline_icon_text_phrase("satellite", "Satellite")}{" an additional reroll."}
                    </>
                },
            },
            Self::UFONavigation => TechInfo {
                name: "UFO Navigation".to_owned(),
                icon: "assets/tech/UFO-navigation.png".to_owned(),
                cost: 2,
                phase: PhaseUsage::Timed,
                description: html! {
                    <>
                    {"Move up to 3 UFOs from any continent to another continent. They must come from the same continent but may be sent to different continents."}
                    </>
                },
            },
            Self::UFOTracking => TechInfo {
                name: "UFO Tracking".to_owned(),
                icon: "assets/tech/UFO-tracking.png".to_owned(),
                cost: 1,
                phase: PhaseUsage::Timed,
                description: html! {
                    <>
                    {"Move up to 2 deployed "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" from any continent to any other continent."}
                    </>
                },
            },
            Self::Firestorm => TechInfo {
                name: "Firestorm".to_owned(),
                icon: "assets/tech/firestorm.png".to_owned(),
                cost: 3,
                phase: PhaseUsage::Resolution,
                description: html! {
                    <>
                    {"Remove one UFO from a continent with at least one "}{inline_icon_text_phrase("interceptor", "Interceptor")}{" deployed to it."}
                    </>
                },
            },
            Self::EMPCannon => TechInfo {
                name: "EMP Cannon".to_owned(),
                icon: "assets/tech/EMP-cannon.png".to_owned(),
                cost: 2,
                phase: PhaseUsage::Resolution,
                description: html! {
                    <>
                    {"Before resolving an attempt at a task, immediately reroll any amount of "}{inline_icon_text_phrase("success", "Success Dice.")}{" Only resolve any "}{inline_icon_text_phrase("success", "Successes")}{" showing after the reroll - you cannot reroll a "}{inline_icon_text_phrase("success", "Success")}{" to try and get multiple "}{inline_icon_text_phrase("success", "Successes")}{" from the same die."}{" This may be used in conjunction with "}{inline_icon_text_phrase("tech", "Carapace Armour.")}
                    </>
                },
            },
            Self::DefenceMatrix => TechInfo {
                name: "Defence Matrix".to_owned(),
                icon: "assets/tech/defence-matrix.png".to_owned(),
                cost: 1,
                phase: PhaseUsage::Resolution,
                description: html! {
                    <>
                    {"When you would lose any amount of "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" during the "}{inline_icon_text_phrase("success", "UFO Defense")}{" task, lose one fewer "}{inline_icon_text_phrase("interceptor", "Interceptor.")}{" This can be used to reduce the number of lost "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" from one to zero, but may only be used once per round."}
                    </>
                },
            },
            Self::EleriumGenerator => TechInfo {
                name: "Elerium Generator".to_owned(),
                icon: "assets/tech/elerium-generator.png".to_owned(),
                cost: 3,
                phase: PhaseUsage::Timed,
                description: html! {
                    <>
                    {"Add §2 to your funds from the supply."}
                    </>
                },
            },
            Self::CarapaceArmour => TechInfo {
                name: "Carapace Armour".to_owned(),
                icon: "assets/tech/carapace-armour.png".to_owned(),
                cost: 2,
                phase: PhaseUsage::Resolution,
                description: html! {
                    <>
                    {"Before resolving an attempt at a task, immediately reroll the "}{inline_icon_text_phrase("alien", "Alien Die")}{" and use the new result instead. This can be used to avoid the negative consequences of the "}{inline_icon_text_phrase("alien", "Alien Threat.")}{" This may be used in conjunction with "}{inline_icon_text_phrase("tech", "EMP Cannon.")}
                    </>
                },
            },
            Self::AlienIntel => TechInfo {
                name: "Alien Intel".to_owned(),
                icon: "assets/tech/alien-intel.png".to_owned(),
                cost: 1,
                phase: PhaseUsage::Resolution,
                description: html! {
                    <>
                    {"Before rolling an attempt at a task, reduce the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" by one space. This cannot be used to reduce "}{inline_icon_text_phrase("alien", "Alien Threat")}{" to below one."}
                    </>
                },
            },
        }
    }
}

pub struct TechReference {
    selected_tech: Option<Tech>,
}

pub enum Msg {
    SelectTech(Tech),
}

impl Component for TechReference {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_tech: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectTech(tech) => {
                self.selected_tech = Some(tech);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="tech-ref-grid">
                    {
                        Tech::get_all().into_iter().map(|tech| html!{
                            <img
                                class={classes!(
                                    "tech-icon",
                                    matches!(&self.selected_tech, Some(t) if *t == tech).as_some("selected")
                                )}
                                onclick={ctx.link().callback(move |_| Msg::SelectTech(tech.clone()))}
                                src={tech.get_info().icon}
                            />
                        }).collect::<Html>()
                    }
                </div>
                <div>
                    {
                        if let Some(ref tech) = self.selected_tech {
                            tech.get_info().render_details()
                        } else {
                            html!{
                                <div>
                                    <p class="tech-description">
                                        {"Select a "}{inline_icon_text_phrase("tech", "Technology")}{" for details."}
                                    </p>
                                </div>
                            }
                        }
                    }
                </div>
            </>
        }
    }
}
