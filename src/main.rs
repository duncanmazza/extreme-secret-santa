use dioxus::prelude::*;
use std::collections::HashMap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Define the types of answers we support
#[derive(Clone, PartialEq)]
enum AnswerType {
    MultipleChoice(&'static [&'static str]),
    OpenResponse,
}

// Structure to hold question data
#[derive(Clone, PartialEq)]
struct Question {
    id: &'static str,
    text: &'static str,
    answer_type: AnswerType,
    correct_answer: &'static str,
}

// Define our questions
const QUESTIONS: &[Question] = &[
    Question {
        id: "q1",
        text: "What is the capital of France?",
        answer_type: AnswerType::OpenResponse,
        correct_answer: "Paris",
    },
    Question {
        id: "q2",
        text: "Which planet is closest to the Sun?",
        answer_type: AnswerType::MultipleChoice(&[
            "Venus",
            "Mercury",
            "Mars",
        ]),
        correct_answer: "Mercury",
    },
];

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Track answers using signal instead of ref
    let answers = use_signal(|| HashMap::<String, String>::new());
    
    // Calculate if all questions are answered correctly
    let all_correct = QUESTIONS.iter().all(|q| {
        answers
            .read()
            .get(q.id)
            .map_or(false, |a| a.trim().eq_ignore_ascii_case(q.correct_answer))
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

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

            // Show success message when all answers are correct
            {
                if all_correct {
                    rsx! {
                        div { class: "success-or-failure-message success", "🎉 The combination lock passcode is ***REMOVED***" }
                    }
                } else {
                    rsx! {
                        div { class: "success-or-failure-message failure", "Not all questions have been answered correctly " }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionCard(
    question: Question, 
    answers: Signal<HashMap<String, String>>
) -> Element {
    let mut answer = use_signal(String::new);

    let mut handle_answer = move |new_answer: String| {
        answers.write().insert(question.id.to_string(), new_answer.clone());
        answer.set(new_answer);
    };

    // let is_correct = answers
    //     .read()
    //     .get(question.id)
    //     .map_or(false, |a| a == &question.correct_answer);

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