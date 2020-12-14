use anyhow::anyhow;
//use pdf_canvas::graphicsstate::Color;
//use pdf_canvas::{BuiltinFont, FontSource, Pdf};
use rand::prelude::*;

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
    println!("{:?}",vec_quest);


    anyhow::Result::Ok(())
}
