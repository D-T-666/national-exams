use core::panic;
use std::convert::identity;
use std::env;
use std::num::ParseIntError;
use std::{collections::HashMap, path::Path};

use itertools::Itertools;

use csv::{ReaderBuilder, StringRecord, Writer};

mod parsing;
mod processing;
mod read;
use parsing::*;
use processing::*;
use read::*;

// mod crate;

// bin <path_to_sorted_scores> <path_to_independent_descaler_data>
fn main() {
    let args: Vec<String> = env::args().collect();

    let publication_file_name = args[1].as_str();

    let (students, schools, faculties) = read_publication_tsv(publication_file_name);

    let independent_data_file = args[2].as_str();

    let independent_descaling_data = read_independent_descaling_data(independent_data_file);

    let students = descale_with_independent_data(students, independent_descaling_data);
    let students = sort_students(students);

    for s in students {
        // println!("{:?}", s);

        let student_string = [
            s.placement.unwrap().to_string(),
            format!(
                "{{{}}}",
                s.scores
                    .iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(i, s)| match s {
                        Some(s) =>
                            Some(format!("{}:{}", ALL_SUBJECTS[i].to_string(), s.to_string())),
                        None => None,
                    })
                    .join(";")
            ),
            s.overall_score,
            s.id,
            schools
                .get(&String::from(&s.faculty_id[0..3]))
                .unwrap()
                .name
                .clone(),
            faculties.get(&s.faculty_id).unwrap().name.clone(),
            match s.grant {
                Some(e) => e.to_string(),
                None => String::from(""),
            },
        ]
        .join(",");

        println!("{student_string}");
    }

    // let args: Vec<String> = env::args().collect();

    // let scores_file_name = args[1].as_str();
    // let mut csv_reader = ReaderBuilder::new()
    //     .delimiter(b'\t')
    //     .from_path(scores_file_name)
    //     .expect("couldn't read the scores file");

    // let mut subject_data = [SubjectStats; ALL_SUBJECTS.len()];

    // while !csv_reader.is_done() {
    //     let mut csv_line = StringRecord::new();
    //     csv_reader
    //         .read_record(&mut csv_line)
    //         .expect("error while reading");
    //     let csv_line = csv_line.iter().map(String::from).collect_vec();

    //     if csv_line.len() == 0 {
    //         break;
    //     }

    //     let (student_data, school, faculty) = parse_student(csv_line);

    //     for (i, subject) in ALL_SUBJECTS.iter().enumerate() {
    //         let score = student_data.scores[i];
    //     }

    //     for scaled_score in &student_data.scores {
    //         let subject = scaled_score.subject;

    //         if !subject_data.contains_key(&subject) {
    //             subject_data.insert(subject, SubjectData::new());
    //             let Some(va) = subject_data.get_mut(&subject) else {
    //                 panic!("SHIT")
    //             };
    //             va.min_scaled = scaled_score.score;
    //             va.max_scaled = scaled_score.score;
    //         } else {
    //             let Some(va) = subject_data.get_mut(&subject) else {
    //                 panic!("SHIT")
    //             };

    //             va.min_scaled = f32::min(va.min_scaled, scaled_score.score);
    //             va.max_scaled = f32::max(va.max_scaled, scaled_score.score);
    //         }
    //     }
    // }

    // // READ INDEPENDENT DATA

    // let independent_data_file_name = args[2].as_str();
    // let mut csv_reader = ReaderBuilder::new()
    //     .from_path(independent_data_file_name)
    //     .expect("couldn't read the independent data file");

    // while !csv_reader.is_done() {
    //     let mut csv_line = StringRecord::new();
    //     csv_reader
    //         .read_record(&mut csv_line)
    //         .expect("error while reading independent data");
    //     let csv_line = csv_line.iter().collect_vec();

    //     if csv_line.len() == 0 {
    //         break;
    //     }

    //     let Some(subject) = subject_from_string(csv_line[0]) else {
    //         panic!("cmon")
    //     };

    //     if !subject_data.contains_key(&subject) {
    //         subject_data.insert(subject, SubjectData::new());
    //     }

    //     let Some(va) = subject_data.get_mut(&subject) else {
    //         panic!("SHIT")
    //     };

    //     match csv_line[1] {
    //         "maximum" => va.max = csv_line[2].parse().expect("invalid number"),
    //         "minimum" => va.min = csv_line[2].parse().expect("invalid number"),
    //         "anchor" => va.anchors.push(SubjectDataAnchor {
    //             raw: csv_line[2].parse().expect("invalid number"),
    //             scaled: csv_line[3].parse().expect("invalid number"),
    //         }),
    //         _ => todo!(),
    //     }
    // }

    // for (k, v) in subject_data.iter_mut() {
    //     if v.min == 0.0 {
    //         v.min = v.max * 0.2;
    //     }
    //     println!("subject: {k:?}, value {v:?}");
    // }

    // // WRITE OUT

    // let mut writer = Writer::from_path("data/scores-descaled.csv").expect("couldn't open a writer");
    // let mut reader = ReaderBuilder::new()
    //     .delimiter(b'\t')
    //     .from_path(args[1].as_str())
    //     .expect("couldn't read the file");

    // while !reader.is_done() {
    //     let mut record = StringRecord::new();
    //     reader.read_record(&mut record).unwrap();
    //     let mut record = record.iter().collect_vec();

    //     if record.len() == 0 {
    //         continue;
    //     }

    //     let score_string: String = record[1].to_string();

    //     let score_string = score_string
    //         .split("-")
    //         .map(|a| {
    //             a.strip_prefix("(")
    //                 .expect("no '('")
    //                 .strip_suffix(")")
    //                 .expect("no ')'")
    //                 .split(",")
    //                 .collect_tuple()
    //                 .expect("cannot tupleate")
    //         })
    //         .map(|(score, subject)| {
    //             let mut score = score.parse().expect("invalid score");

    //             if let Some(subject) = subject_from_string(subject) {
    //                 let subject_data = subject_data.get(&subject).expect("no data about subject");

    //                 if subject_data.anchors.len() == 0 {
    //                     score = map_range(
    //                         score,
    //                         subject_data.min_scaled,
    //                         subject_data.max_scaled,
    //                         subject_data.min,
    //                         subject_data.max,
    //                     );
    //                 } else {
    //                     let mut min_scaled = subject_data.min_scaled;
    //                     let mut max_scaled = subject_data.max_scaled;
    //                     let mut min = subject_data.min;
    //                     let mut max = subject_data.max;

    //                     for anchor in &subject_data.anchors {
    //                         if anchor.scaled <= score {
    //                             min_scaled = anchor.scaled;
    //                             min = anchor.raw;
    //                         }
    //                     }

    //                     for anchor in &subject_data.anchors {
    //                         if score < anchor.scaled {
    //                             max_scaled = anchor.scaled;
    //                             max = anchor.raw;
    //                         }
    //                     }

    //                     score = map_range(score, min_scaled, max_scaled, min, max);
    //                 }
    //             };

    //             format!("{}:{:.2}", subject, score)
    //         })
    //         .join(";");

    //     let score_string = format!("{{{}}}", score_string.as_str());
    //     record[1] = score_string.as_str();
    //     writer.write_record(record).unwrap();
    // }

    // writer.flush().unwrap();
}
