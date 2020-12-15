use anyhow::anyhow;
use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, FontSource, Pdf};
use rand::prelude::*;

fn blank_splace(to_serch: &String) -> usize {
    let mut base = 70;
    let v: Vec<&str> = to_serch.split("").collect();
    while v[base] != " " {
        base -= 1;
    }
    base
}
fn randomize_answers(answers: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut sanswers: Vec<&str> = answers.split("-").collect();
    sanswers.shuffle(&mut rng);
    let mut returned_string: Vec<String> = Vec::new();
    let mut nb_quest = 1;
    //boucle infinie
    /*while let Some(join_mode) = sanswers.iter().next() {
        returned_string.push(format!("{}){}", nb_quest, join_mode));
        nb_quest += 1;
        println!("hum");
    }*/
    for i in 0..sanswers.len(){
        returned_string.push(format!("{}){} ", nb_quest, sanswers[i]));
        nb_quest += 1;
    }
    returned_string.join(" ")
}
fn create_qcm(num: i32, mut content: Vec<String>) {
    for n in 1..=num {
        let file_name = format!("../qcm_folder/qcm{}.pdf", n);

        let mut document = Pdf::create(&file_name).expect("Create pdf file");

        let font = BuiltinFont::Times_Roman;

        while !content.is_empty() {
            let mut height_to_right = 280.0;
            document
                .render_page(210.0, 297.0, |canvas| {
                    
                    while !content.is_empty() && height_to_right > 40.0 {
                        if let Some(mut hello) = content.pop() {
                            let questy: Vec<&str> = hello.split("answers").collect();
                            let answer_shuf = randomize_answers(questy[1]);
                            if questy[0].len() >= 70 {
                                //need to split_inclusive the qiestion from the answers

                                let reste = questy[0]
                                    .to_string()
                                    .split_off(blank_splace(&questy[0].to_string()));

                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &questy[0])
                                    .expect("this is gonna work");
                                height_to_right -= 5.0;

                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &reste)
                                    .expect("maybe gonna work");

                                height_to_right -= 10.0;

                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &answer_shuf)
                                    .expect("maybe gonna work");
                                height_to_right -= 15.0;
                            } else {
                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &questy[0])
                                    .expect("please work");

                                height_to_right -= 10.0;

                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &answer_shuf)
                                    .expect("maybe gonna work");
                                height_to_right -= 15.0;
                            }
                        }
                        height_to_right -= 10.0;
                    }
                    Ok(())
                })
                .expect("Write page");
        }
        document.finish().expect("Finish pdf document");
    }
}
fn main() -> anyhow::Result<()> {
    let buffer = std::fs::read_to_string("qcm_answer.txt")?;
    let mut lines = buffer.lines();
    let mut rng = rand::thread_rng();
    let mut vec_quest: Vec<String> = Vec::new();
    let how_much = 5;

    while let (Some(mut question), Some(mut reponse)) = (lines.next(), lines.next()) {
        while question == "" {
            question = lines.next().ok_or(anyhow!("Whut happend?"))?;
            let tmp = question;
            question = reponse;
            reponse = tmp;
        }
        let res = format!("{} {}", question, reponse);
        vec_quest.push(res.to_string());
    }

    vec_quest.shuffle(&mut rng);
    //  println!("{:?}",vec_quest);

    create_qcm(how_much, vec_quest);

    anyhow::Result::Ok(())
}
