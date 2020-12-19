use anyhow::anyhow;
use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, FontSource, Pdf};
use rand::prelude::*;

/*fn blank_splace(mut base: usize, to_serch: &String) -> usize {
    //let mut base = 70;
    let v: Vec<&str> = to_serch.split("").collect();
    while v[base] != " " {
        base -= 1;
    }
    base
}
*/
fn blank_splace_plus(mut base: usize, to_serch: &String) -> usize {
    //let mut base = 70;
    let v: Vec<&str> = to_serch.split("").collect();
    while v[base] != " " {
        base += 1;
    }
    base
}
fn randomize_answers(answers: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut sanswers: Vec<&str> = answers.split("-").collect();
    sanswers.shuffle(&mut rng);
    let mut returned_string: Vec<String> = Vec::new();
    let mut nb_quest = 1;

    //println!("{:?}",sanswers);
    sanswers.retain(|&x| x != " ");

    for i in 0..sanswers.len() {
        returned_string.push(format!("{}){} ", nb_quest, sanswers[i]));
        nb_quest += 1;
    }
    returned_string.join(" ")
}

fn create_qcm(num: i32, content: &mut Vec<String>) {
    let file_name = format!("../qcm_folder/qcm{}.pdf", num);

    let mut document = Pdf::create(&file_name).expect("Create pdf file");

    let font = BuiltinFont::Times_Roman;

    while !content.is_empty() {
        let mut height_to_right = 280.0;
        document
            .render_page(210.0, 297.0, |canvas| {
                canvas
                    .left_text(10.0, height_to_right, font, 6.0, "NAME:")
                    .expect("this is gonna work");
                height_to_right -= 10.0;
                canvas
                    .left_text(10.0, height_to_right, font, 6.0, "DATE:")
                    .expect("this is gonna work");
                height_to_right -= 20.0;
                while !content.is_empty() && height_to_right > 30.0 {
                    if let Some(hello) = content.pop() {
                        let questy: Vec<&str> = hello.split("answers:").collect();
                        let answer_shuf = randomize_answers(questy[1]);

                        if questy[0].len() > 70 {
                            // let mut vec_part_questy:Vec<String>=Vec::new();
                            let mut the_good_vec: Vec<String> = Vec::new();
                            let mut clone_questy = questy[0].to_string().clone();
                            while clone_questy.len() > 70 {
                                the_good_vec.push(clone_questy.split_off(blank_splace_plus(
                                    clone_questy.len() - 70,
                                    &clone_questy,
                                )));
                            }

                            canvas
                                .left_text(10.0, height_to_right, font, 6.0, &clone_questy)
                                .expect("this is gonna work");
                            height_to_right -= 5.0;

                            /*canvas
                            .left_text(10.0, height_to_right, font, 6.0, &reste)
                            .expect("maybe gonna work");
                            */
                            while let Some(muda) = the_good_vec.pop() {
                                println!("{}", muda);
                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &muda)
                                    .expect("I need coffee");
                                height_to_right -= 5.0;
                            }
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
fn main() -> anyhow::Result<()> {
    let buffer = std::fs::read_to_string("qcm_answer.txt")?;
    let mut lines = buffer.lines();
    let mut rng = rand::thread_rng();
    let mut vec_quest: Vec<String> = Vec::new();
    let mut vec_vec: Vec<Vec<String>> = Vec::new();
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
    for i in 1..=how_much {
        vec_quest.shuffle(&mut rng);
        let to_push_vec = vec_quest.clone();
        vec_vec.push(to_push_vec);
        //println!("{:?}",vec_quest);
    }

    for n in 1..=how_much {
        create_qcm(n, &mut vec_vec[(n - 1) as usize]);
    }
    anyhow::Result::Ok(())
}
