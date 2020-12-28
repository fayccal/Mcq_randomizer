mod fonc;
use anyhow::anyhow;
use rand::prelude::*;

fn main() -> anyhow::Result<()> {
    // Récupère le contenu du fichiers
    let buffer = std::fs::read_to_string("qcm_answer.txt")?;
    let mut lines = buffer.lines();
    let mut rng = rand::thread_rng();
    let mut vec_quest: Vec<String> = Vec::new();
    let mut vec_vec: Vec<Vec<String>> = Vec::new();
    // Combien de fichiers voulu
    let how_much = 5;

    while let (Some(mut question), Some(mut reponse)) = (lines.next(), lines.next()) {
        while question == "" {
            question = lines.next().ok_or(anyhow!("Whut happend?"))?;
            let tmp = question;
            question = reponse;
            reponse = tmp;
        }
        // On combine les questions et réponses que chaqu'une
        let res = format!("{} {}", question, reponse);
        vec_quest.push(res.to_string());
    }

    for _i in 1..=how_much {
        // Besoin de cloné le vec car il est consumé
        vec_quest.shuffle(&mut rng);
        let to_push_vec = vec_quest.clone();
        vec_vec.push(to_push_vec);
    }

    // Génération des qcm
    for n in 1..=how_much {
        fonc::create_qcm(n, &mut vec_vec[(n - 1) as usize])?;
    }
    anyhow::Result::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    ///test if we shuffuled (maybe false because it could give the same one even after the shuffle)
    fn test_shuffle() {
        let mut rng = rand::thread_rng();
        let mut sample = vec!["une ligne", "une deuxieme", "et une troisième"];
        sample.shuffle(&mut rng);

        assert_ne!(sample, ["une ligne", "une deuxieme", "et une troisième"]);
    }

    #[test]
    ///test if we returned the white space position in the question to be cut after
    fn test_blank_space() {
        let sample = "Hello i'am a sample, i'm here to be tested, happy to work with you now let's get started".to_string();
        assert_eq!(67, fonc::blank_space(70, &sample));
    }
}
