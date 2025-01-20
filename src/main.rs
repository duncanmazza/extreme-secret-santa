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
}

lazy_static::lazy_static! {
    static ref QUESTIONS: Vec<Question> = {
        let questions = vec![
            Question {
                id: 0,
                text: "What is the capital of France?",
                answer_type: AnswerType::OpenResponse,
                correct_answer: "Paris",
            },
            Question {
                id: 0,
                text: "Which planet is closest to the Sun?",
                answer_type: AnswerType::MultipleChoice(&[
                    "Venus",
                    "Mercury",
                    "Mars",
                ]),
                correct_answer: "Mercury",
            },
        ];
        questions.into_iter().enumerate()
            .map(|(i, mut q)| {
                q.id = (i + 1) as i32;
                q
            })
            .collect()
    };
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let answers = use_signal(|| HashMap::<i32, String>::new());

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
            href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎅</text></svg>",
        }

        div { class: "container",
            h1 { class: "title", "Trivia Challenge" }

            p { class: "body", "TODO" }

            div { class: "questions",
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

            {
                if all_correct {
                    rsx! {
                        div { class: "success-or-failure-message success", "🎉 The combination lock passcode is ***REMOVED***" }
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

#[component]
fn QuestionCard(question: Question, answers: Signal<HashMap<i32, String>>) -> Element {
    let mut answer = use_signal(String::new);

    let mut handle_answer = move |new_answer: String| {
        answers.write().insert(question.id, new_answer.clone());
        answer.set(new_answer);
    };

    rsx! {
        div { class: "question-card",
            h3 { class: "question-text", "{question.text}" }

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
