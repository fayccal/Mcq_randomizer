use pdf_canvas::{BuiltinFont, Pdf};
use rand::prelude::*;

///fonction pour split sur un espace blanc au retour à la ligne (utilisation suspendu)
/*fn blank_splace(mut base: usize, to_serch: &String) -> usize {
    //let mut base = 70;
    let v: Vec<&str> = to_serch.split("").collect();
    while v[base] != " " {
        base -= 1;
    }
    base
}
*/

///fonction pour split sur un espace blanc au retour à la ligne
pub fn blank_splace_plus(mut base: usize, to_serch: &String) -> usize {
    let v: Vec<&str> = to_serch.split("").collect();
    while v[base] != " " {
        base += 1;
    }
    base
}

///randomize les réponse à une question
pub fn randomize_answers(answers: &str) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut sanswers: Vec<&str> = answers.split("-").collect();
    sanswers.shuffle(&mut rng);
    let mut returned_string: Vec<String> = Vec::new();
    let mut nb_quest = 1;

    sanswers.retain(|&x| x != " ");

    for i in 0..sanswers.len() {
        returned_string.push(format!("{}){} ", nb_quest, sanswers[i]));
        nb_quest += 1;
    }
    returned_string
}

///fonction de création du qcm
pub fn create_qcm(num: i32, content: &mut Vec<String>) {
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
                            the_good_vec.reverse();
                            /*  let test:Vec<&str>=questy[0].clone().split_whitespace().collect();
                                while !test.is_empty() {
                                while test[0].len()+test[1].len() < 70 {
                                    [test[0], test[1]].join(" ");
                                }
                                the_good_vec.push(test[0].to_string());
                            }*/

                            //split tout et créée des phrase
                            canvas
                                .left_text(10.0, height_to_right, font, 6.0, &clone_questy)
                                .expect("this is gonna work");
                            height_to_right -= 5.0;

                            /*canvas
                            .left_text(10.0, height_to_right, font, 6.0, &reste)
                            .expect("maybe gonna work");
                            */
                            /*while let Some(muda) = the_good_vec.pop() {
                                println!("{}", muda);
                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &muda)
                                    .expect("I need coffee");
                                height_to_right -= 5.0;
                            }*/

                            for i in the_good_vec {
                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &i)
                                    .expect("I need coffee");
                                height_to_right -= 5.0;
                            }
                            height_to_right -= 10.0;

                            /*canvas
                                .left_text(10.0, height_to_right, font, 6.0, &answer_shuf)
                                .expect("maybe gonna work");
                            height_to_right -= 15.0;*/

                            for j in answer_shuf {
                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &j)
                                    .expect("maybe gonna work");
                                height_to_right -= 5.0;
                            }
                            height_to_right -= 10.0;
                        } else {
                            canvas
                                .left_text(10.0, height_to_right, font, 6.0, &questy[0])
                                .expect("please work");

                            height_to_right -= 10.0;

                            /*canvas
                                .left_text(10.0, height_to_right, font, 6.0, &answer_shuf)
                                .expect("maybe gonna work");
                            height_to_right -= 15.0;*/
                            for j in answer_shuf {
                                canvas
                                    .left_text(10.0, height_to_right, font, 6.0, &j)
                                    .expect("maybe gonna work");
                                height_to_right -= 5.0;
                            }
                            height_to_right -= 10.0;
                        }
                    }
                    height_to_right -= 5.0;
                }
                Ok(())
            })
            .expect("Write page");
    }
    document.finish().expect("Finish pdf document");
}
