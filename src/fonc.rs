use pdf_canvas::{BuiltinFont, Pdf};
use rand::prelude::*;
use std::io;
use std::mem::swap;

///Fonction pour split sur un espace blanc au retour à la ligne
///Function to split on a white space
pub fn blank_space(mut base: usize, to_search: &String) -> usize {
    if base > to_search.len() {
        base = to_search.len();
    }
    //Cherche un espace blanc pour ne pas couper de mot
    //Search a blank space
    let v: Vec<&str> = to_search.split("").collect();
    while v[base] != " " {
        base -= 1;
    }
    base
}

///Randomize les réponses à une question
///Randomize the answers of a question
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

///Fonction de création du qcm
///Function creating the mcq
pub fn create_qcm(num: i32, content: &mut Vec<String>) -> io::Result<()> {
    let file_name = format!("../qcm_folder/qcm{}.pdf", num);

    let mut document = Pdf::create(&file_name)?;

    let font = BuiltinFont::Times_Roman;

    //Temps que on a encore du contenu on boucle
    //While we have some content
    while !content.is_empty() {
        let mut height_to_right = 280.0;
        document.render_page(210.0, 297.0, |canvas| {
            canvas.left_text(10.0, height_to_right, font, 6.0, "NAME:")?;
            height_to_right -= 10.0;
            canvas.left_text(10.0, height_to_right, font, 6.0, "DATE:")?;
            height_to_right -= 20.0;
            //Si on va pas trop en bas de la page et que on a encore du contenu
            //If we are to much below the page and we have some content
            while !content.is_empty() && height_to_right > 30.0 {
                if let Some(hello) = content.pop() {
                    let questy: Vec<&str> = hello.split("answers:").collect();
                    let answer_shuf = randomize_answers(questy[1]);

                    if questy[0].len() > 70 {
                        let mut the_good_vec: Vec<String> = Vec::new();
                        let mut clone_questy = questy[0].to_string().clone();
                        let mut cara_count = 0;
                        let mut nb_line = 0;
                        while cara_count <= clone_questy.len() {
                            cara_count += 70;
                            nb_line += 1;
                        }
                        //Combien de ligne on aura a écrire
                        //How much line do we have write
                        for _i in 0..nb_line {
                            let mut reste = clone_questy.split_off(blank_space(70, &clone_questy));
                            swap(&mut clone_questy, &mut reste);
                            the_good_vec.push(reste);
                        }

                        //On écrit la question
                        //We write the question
                        for i in the_good_vec {
                            canvas.left_text(10.0, height_to_right, font, 6.0, &i)?;
                            height_to_right -= 5.0;
                        }
                        height_to_right -= 10.0;
                        //On écrit les réponses
                        //We write the answers
                        for j in answer_shuf {
                            canvas.left_text(10.0, height_to_right, font, 6.0, &j)?;
                            height_to_right -= 5.0;
                        }
                        height_to_right -= 10.0;
                    } else {
                        //Si il y a seulement une ligne de question
                        //If there is only one line for the question
                        canvas.left_text(10.0, height_to_right, font, 6.0, &questy[0])?;

                        height_to_right -= 10.0;

                        for j in answer_shuf {
                            canvas.left_text(10.0, height_to_right, font, 6.0, &j)?;
                            height_to_right -= 5.0;
                        }
                        height_to_right -= 10.0;
                    }
                }
                height_to_right -= 5.0;
            }
            Ok(())
        })?;
    }
    document.finish()?;
    Ok(())
}
