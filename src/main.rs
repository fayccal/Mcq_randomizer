use anyhow::anyhow;
use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, FontSource, Pdf};
use rand::prelude::*;

fn blanck_splace(to_serch:&String) -> usize{
    let mut base=70;
    let v:Vec<&str> = to_serch.split("").collect();
    while v[base] !=" " {
        base-=1;
    }
    base
}

fn create_qcm(num: i32, mut content: Vec<String>) {
    for n in 1..=num {
        let file_name = format!("../qcm_folder/qcm{}.pdf", n);

        let mut document = Pdf::create(&file_name).expect("Create pdf file");

        let font = BuiltinFont::Times_Roman;

        let mut height_to_right = 280.0;
        document
            .render_page(210.0, 297.0, |canvas| {
                while !content.is_empty() {
                    // Some text
                    if let Some(mut hello) = content.pop() {
                        if hello.len() >= 70 {
                            let reste = hello.split_off(blanck_splace(&hello));
                            
                            canvas
                                .left_text(10.0, height_to_right, font, 6.0, &hello)
                                .expect("this is gonna work");
                            height_to_right -= 10.0;
                            
                            canvas
                                .left_text(10.0, height_to_right, font, 6.0, &reste)
                                .expect("maybe gonna work");
                        }
                        else{
                        canvas
                            .left_text(10.0, height_to_right, font, 6.0, &hello)
                            .expect("please work");
                        //1 si la taille d'une certain phrase est trop long on la coup a l'extrémité(70)
                        //2 page add automatique si trop de question
                        }
                    }
                    height_to_right -= 10.0;
                }
                Ok(())
            })
            .expect("Write page");
        document.finish().expect("Finish pdf document");
    }
}
fn main() -> anyhow::Result<()> {
    let buffer = std::fs::read_to_string("base_qcm.txt")?;
    let mut lines = buffer.lines();
    let mut rng = rand::thread_rng();
    let mut vec_quest: Vec<String> = Vec::new();
    let how_much = 5;

    while let Some(mut question) = lines.next() {
        while question == "" {
            question = lines.next().ok_or(anyhow!("Whut happend?"))?;
        }
        vec_quest.push(question.to_string());
    }

    vec_quest.shuffle(&mut rng);
    //  println!("{:?}",vec_quest);

    create_qcm(how_much, vec_quest);

    anyhow::Result::Ok(())
}
