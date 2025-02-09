#![allow(non_snake_case)]
use ::extreme_secret_santa::obf::deobfuscate;
use dioxus::prelude::*;
use std::collections::HashMap;

extern crate lazy_static;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

// Define the types of answers we support
#[derive(Clone, PartialEq)]
enum AnswerType {
    MultipleChoice(&'static [&'static str]),
    OpenResponse,
}

// Structure to hold question data
#[derive(Clone, PartialEq)]
struct Question {
    id: i32,
    text: &'static str,
    answer_type: AnswerType,
    correct_answer: &'static str,
    image: Option<Asset>,
}

lazy_static::lazy_static! {
    static ref QUESTIONS: Vec<Question> = {
        let questions = vec![
            Question {
                id: 0,
                text: "What roller coaster is closest to an establishment that offers green milk on its menu?",
                answer_type: AnswerType::OpenResponse,
                correct_answer: "Slinky Dog Dash",
                image: None,
            },
            Question {
                id: 0,
                text: "In the World Showcase, a piece of artwork can be found on the side of a building featuring a diety;
                among the various names this diety goes by, which is the longest uninterrupted by whitespace (according to 
                Wikipedia)?",
                answer_type: AnswerType::OpenResponse,
                correct_answer: "Tlahuizcalpantecuhtli",
                image: None,
            },
            Question {
                id: 0,
                text: "Which hotel has this courtyard?",
                answer_type: AnswerType::MultipleChoice(&[
                    "Port Orleans - Riverside",
                    "Port Orleans - French Quarter",
                    "Art of Animation",
                    "Coronado Sprints",
                ]),
                correct_answer: "Port Orleans - French Quarter",
                image: Some(asset!("/assets/images/french_quarter.png")),
            },
            Question {
                id: 0,
                text: "At what golf course hole number can you find the west-most Mickey Mouse-Shaped sand pit?",
                answer_type: AnswerType::OpenResponse,
                correct_answer: "6",
                image: Some(asset!("/assets/images/mickey_mouse_sand_trap.png")),
            },
            Question {
                id: 0,
                text: "Approximately how much area is covered by the visitor parking lots just north of Epicot? (Not
                including lots for buses.)",
                answer_type: AnswerType::MultipleChoice(
                    &[
                        "320329 m^2",
                        "230932 m^2",
                        "1142009 m^2",
                        "11420 m^2",
                    ]
                ),
                correct_answer: "320329 m^2",
                image: None
            },
            Question {
                id: 0,
                text: "According to the only 1-star review on Google Reviews of the Mickey Mouse-shaped solar farm,
                what should Disney have done instead?",
                answer_type: AnswerType::MultipleChoice(
                    &[
                        "Build a subcritical coal power plant",
                        "Extract enenrgy from roller coasters using regenerative braking",
                        "Build solar panels over the parking lots to minimize additional land use",
                        "Build a rectangular solar farm, because no one cares about the aerial view of a solar farm",
                    ]
                ),
                correct_answer: "Build solar panels over the parking lots to minimize additional land use",
                image: Some(asset!("/assets/images/solar_farm.jpg")),
            },
            Question {
                id: 0,
                text: "If you were to demolish the Epicot Sphere and build a circular solar farm in its footprint,
                what annual energy intensity would you achieve? (Hint: use shademap.app)",
                answer_type: AnswerType::MultipleChoice(
                    &[
                        "345 kWh/m^2",
                        "435 kWh/m^2",
                        "565 kWh/m^2",
                        "655 kWh/m^2",
                    ]
                ),
                correct_answer: "565 kWh/m^2",
                image: None
            },
            Question {
                id: 0,
                text: "If C-3PO hobbles along at 2mph, how many hours would it take him to walk around the hourglass
                lake?",
                answer_type: AnswerType::MultipleChoice(
                    &[
                        "0.42",
                        "0.63",
                        "0.72",
                        "0.95",
                    ]
                ),
                correct_answer: "0.63",
                image: Some(asset!("/assets/images/hourglass_lake.png")),
            },
            Question {
                id: 0,
                text: "What is the name of the open-source scripting language Disney developed?",
                answer_type: AnswerType::OpenResponse,
                correct_answer: "Groovity",
                image: None
            },
            Question {
                id: 0,
                text: "At which hotel can you find giant Duncan's Yo-Yo Tops?",
                answer_type: AnswerType::MultipleChoice(&[
                    "All-Star Movies",
                    "All-Star Music",
                    "All-Star Sports",
                    "Pop Century",
                ]),
                correct_answer: "Pop Century",
                image: None,
            },
            ];

        for question in questions.iter() {
            if let AnswerType::MultipleChoice(multiple_choice) = question.answer_type {
                assert!(multiple_choice.contains(&question.correct_answer));
            }
        }

        questions.into_iter().enumerate()
            .map(|(i, mut q)| {
                q.id = (i + 1) as i32;
                q
            })
            .collect()
    };
}

lazy_static::lazy_static! {
    static ref PASSWORD: String = deobfuscate("06.1;.04");
}

// ANCHOR: main
fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(app);
}
// ANCHOR_END: main

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

#[component]
fn Index() -> Element {
    let answers = use_signal(|| HashMap::<i32, String>::new());
    let password = PASSWORD.as_str();

    let all_correct = QUESTIONS.iter().all(|q| {
        answers
            .read()
            .get(&q.id)
            .map_or(false, |a| a.trim().eq_ignore_ascii_case(q.correct_answer))
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22>
            <text y=%22.9em%22 font-size=%2290%22>🎅</text></svg>",
        }

        div { class: "container",
            h1 { class: "title", "Julia's Extreme Secret Santa Challenge!" }

            p { class: "body",
                "Use your extensive knowledge of and recent trip to Disney World to access the combination lock
                code and unlock your gift. You are welcome to use any internet resrouces you would like. Good luck!"
            }

            br {}

            div { class: "column-container",
                div { class: "left-column",
                    {
                        QUESTIONS
                            .iter()
                            .map(|question| {
                                rsx! {
                                    QuestionCard { key: question.id, question: question.clone(), answers }
                                }
                            })
                    }
                }

                div { class: "right-column", style: "width: 30%",
                    div {
                        {
                            if all_correct {
                                rsx! {
                                    div { class: "success-or-failure-message success", "🎉 The combination lock passcode is {password}" }
                                }
                            } else {
                                rsx! {
                                    div { class: "success-or-failure-message failure", "Not all questions have been answered correctly" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionCard(question: Question, answers: Signal<HashMap<i32, String>>) -> Element {
    let mut answer = use_signal(String::new);

    let mut handle_answer = move |new_answer: String| {
        answers.write().insert(question.id, new_answer.clone());
        answer.set(new_answer);
    };

    rsx! {
        div { class: "question-card",

            h3 { class: "question-text", "{question.id}. {question.text}" }

            if let Some(image_unwrapped) = question.image {
                img { class: "question-img", src: image_unwrapped }
            }

            div { class: "answer-section",
                match question.answer_type {
                    AnswerType::MultipleChoice(options) => rsx! {
                        div { class: "multiple-choice",
                            {
                                options
                                    .iter()
                                    .map(|option| {
                                        let option_str = *option;
                                        let is_selected = answer() == option_str;
                                        rsx! {
                                            button {
                                                class: if is_selected { "choice-button selected" } else { "choice-button" },
                                                onclick: move |_| handle_answer(option_str.to_string()),
                                                "{option}"
                                            }
                                        }
                                    })
                            }
                        }
                    },
                    AnswerType::OpenResponse => rsx! {
                        input {
                            class: "open-response",
                            r#type: "text",
                            value: "{answer}",
                            oninput: move |evt| handle_answer(evt.value().clone()),
                        }
                    },
                }
            }
        }
    }
}

// ANCHOR: static_routes
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    // Any routes with no dynamic segments in your router will be included in the static routes list
    #[route("/extreme-secret-santa")]
    Index {},
}
