use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::{error::Error, fs::File, io::BufReader, process::exit, collections::BTreeMap, iter::zip, io::{stdin, stdout, prelude::*}};
use inquire::{error::InquireError, Select, ui::{RenderConfig, Attributes, Color, Styled, StyleSheet}};

#[derive(Serialize, Deserialize)]
struct QNA {
    question: BTreeMap<String, Vec<String>>,
    answer: Vec<String>
}

//this function doesn't need to be parsed thus always open "datas.json".
fn read_json() -> Result<QNA, Box<dyn Error>> {
    let file = File::open("datas.json")?;
    let reader = BufReader::new(file);
    let qna = from_reader(reader)?;
    Ok(qna)
}

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();
    
    write!(stdout, "Press \x1b[1m\x1b[33menter\x1b[0m to quit!").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let qna = read_json().unwrap();

    let qnums = qna.question.len();
    let anums = qna.answer.len();

    //can't compere questions and answers if they don't have the same number.
    if qnums != anums {
        println!("\x1b[1m\x1b[33mError\x1b[0m: Hey, your answers are different from your questions! Questions: \x1b[1m\x1b[33m{}\x1b[0m, answers: \x1b[1m\x1b[33m{}\x1b[0m", qnums, anums);
        exit(1)
    }

    let mut points: f32 = 0.;

    let style_sheet = StyleSheet::default()
    .with_fg(Color::LightCyan)
    .with_attr(Attributes::ITALIC);

    let styled_option = Styled::new("->").with_style_sheet(style_sheet);

    for ((q, c), a) in zip(qna.question, qna.answer) {

        let ans: Result<String, InquireError> = Select::new(&q, c.to_vec())
        .with_page_size(20)
        .with_help_message("Choose your answer, my friend!")
        .with_render_config(RenderConfig::default().with_highlighted_option_prefix(styled_option).with_selected_option(Some(style_sheet)).with_scroll_up_prefix(Styled::new("^").with_style_sheet(style_sheet)).with_scroll_down_prefix(Styled::new("v").with_style_sheet(style_sheet)))
        .prompt();

        match ans {
            Ok(choice) => {
                if choice == a {
                    points += 1.;
                    println!("\x1b[1m\x1b[33mRight!\x1b[0m\n")
                } else {
                    println!("\x1b[1m\x1b[31mWrong!\x1b[0m\n")
                }
            },
            Err(e) => {
                println!("There was an \x1b[1m\x1b[33merror\x1b[0m: \x1b[1m\x1b[31m{e}\x1b[0m\n");
                exit(1);
            },
        }
    }

    //marks defined here after all questions were answered.
    let marks: f32 = (100.*points) / qnums as f32;
    
    //grading system, don't read it, it's uncomfortable.
    if marks > 75. {
        println!("\x1b[1m\x1b[33m{marks}%\x1b[0m \x1b[1m\x1b[94mgood job!\x1b[0m\n");
    } else if 50. <= marks && marks <= 75. {
        println!("\x1b[1m\x1b[33m{marks}%\x1b[0m not too good\n");
    } else if 25. <= marks && marks < 50. {
        println!("\x1b[1m\x1b[33m{marks}%\x1b[0m not too good\n");
    } else {
        println!("\x1b[1m\x1b[33m{marks}%\x1b[0m \x1b[1m\x1b[31myou suck!\x1b[0m\n");
    }

    pause();
}