use std::ops::Add;

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error(String);

pub struct Scale(Vec<String>);

#[derive(Debug, PartialEq, Eq)]
enum ChromaticScaleAccidentType {
    None,
    Flat,
    Sharp,
}

impl Scale {
    fn get_chromatic_scale_accident_type(tonic: &str) -> ChromaticScaleAccidentType {
        match tonic {
            "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => {
                ChromaticScaleAccidentType::Sharp
            }
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => {
                ChromaticScaleAccidentType::Flat
            }
            _ => ChromaticScaleAccidentType::None,
        }
    }

    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let chromatic_scale = Scale::chromatic(tonic);
        if chromatic_scale.is_err() {
            // we return the err value
            return chromatic_scale;
        }

        let chromatic_scale_notes = chromatic_scale.unwrap().enumerate();

        let mut new_scale_notes = vec![];
        new_scale_notes.push(Scale::normalize_tonic(tonic).to_owned());

        let mut i = 0;
        for interval in intervals.chars() {
            let offset = match interval {
                'M' => 2,
                'm' => 1,
                'A' => 3,
                _ => return Err(Error("invalid interval".to_owned())),
            };

            i = i + offset;

            match chromatic_scale_notes.get(i) {
                Some(note) => new_scale_notes.push(String::from(note)),
                None => return Err(Error("invalid intervals pattern".to_owned())),
            }
        }

        Ok(Scale(new_scale_notes))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let chromatic_scale_notes_with_sharps: Vec<&'static str> = vec![
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let chromatic_scale_notes_with_flats: Vec<&'static str> = vec![
            "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
        ];

        let notes = if Scale::get_chromatic_scale_accident_type(tonic)
            == ChromaticScaleAccidentType::Flat
        {
            chromatic_scale_notes_with_flats
        } else {
            chromatic_scale_notes_with_sharps
        };

        match notes
            .iter()
            .position(|&note| note.to_lowercase() == tonic.to_lowercase())
        {
            None => Err(Error("tonic not found".to_owned())),
            Some(tonic_idx) => {
                let mut n1: Vec<String> = notes
                    .iter()
                    .skip(tonic_idx)
                    .take(notes.len() - tonic_idx)
                    .map(|&x| x.to_owned())
                    .collect();
                let mut n2: Vec<String> = notes
                    .iter()
                    .take(tonic_idx)
                    .map(|&x| x.to_owned())
                    .collect();

                n1.append(&mut n2);
                n1.push(Scale::normalize_tonic(tonic).to_owned());

                Ok(Scale(n1))
            }
        }
    }

    pub fn enumerate(&self) -> Vec<String> {
        let Scale(notes) = self;

        notes.to_vec()
    }

    fn normalize_tonic(tonic: &str) -> String {
        if tonic.len() > 1 {
            let mut s = String::new();
            s = s.add(&tonic.chars().next().unwrap().to_uppercase().to_string());
            s = s.add(&tonic[1..]);
            s
        } else {
            tonic.to_uppercase().to_owned()
        }
    }
}
