use core::panic;
use std::env;
use std::num::ParseIntError;
use std::{collections::HashMap, path::Path};

use itertools::Itertools;

use csv::{ReaderBuilder, StringRecord, Writer};

mod parsing;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let mut reader = ReaderBuilder::new()
    //     .delimiter(b'\t')
    //     .from_path(args[1].as_str())
    //     .expect("couldn't read the file");

    // let mut data = Vec::new();

    // let mut scores_per_subject = HashMap::new();
    // let mut all_cor: Vec<(String, String)> = Vec::new();

    // while !reader.is_done() {
    //     let mut record = StringRecord::new();
    //     reader
    //         .read_record(&mut record)
    //         .expect("error while reading");
    //     let record = record.iter().collect_vec();
    //     if record.len() == 0 {
    //         break;
    //     }

    //     // println!("{:?}", record);
    //     let student_data = parse_studnet(record).unwrap();
    //     for scaled_score in &student_data.scores {
    //         let name = match scaled_score.subject {
    //             Georgian => "GEO",
    //             English => "უცხოური",
    //             Math => "MATH",
    //             History => "HIST",
    //             Physics => "ფიზიკა",
    //             Chemistry => "ქიმია",
    //             Biology => "ბიოლოგია",
    //             Geography => "გეოგრაფია",
    //         };
    //         insert_into_list_map(name.to_string(), *scaled_score, &mut scores_per_subject);
    //     }
    //     data.push(student_data);
    // }

    // for (name, list) in &mut scores_per_subject {
    //     list.sort_by_key(|(scaled_score, _count)| (scaled_score.score * 10.0) as i32);
    //     let matching_path = format!("data/{}.csv", name);
    //     if Path::new(matching_path.as_str()).exists() {
    //         let mut reader = ReaderBuilder::new()
    //             .from_path(matching_path)
    //             .expect("couldn't read the matching file");
    //         let mut writer = Writer::from_path(format!("data/{}-matched.csv", name))
    //             .expect("couldn't open a writer");

    //         let mut data = Vec::new();

    //         while !reader.is_done() {
    //             let mut record = StringRecord::new();
    //             reader.read_record(&mut record).unwrap();
    //             let record = record.iter().collect_vec();

    //             if record.len() == 0 {
    //                 continue;
    //             }

    //             data.push(DataPoint {
    //                 variant: record[0].parse().unwrap(),
    //                 density: record[1].parse().unwrap(),
    //                 raw_score: record[2].parse().unwrap(),
    //                 equalized: record[3].parse().unwrap(),
    //                 subject: name.clone(),
    //             });
    //         }

    //         let corelation = data.iter().rev().zip(list.iter().rev()).rev().collect_vec();

    //         for (dp, (scaled_score, _)) in corelation {
    //             let subject = match scaled_score.subject {
    //                 Georgian => "ქართული",
    //                 English => "უცხოური",
    //                 Math => "მათემატიკა",
    //                 History => "ისტორია",
    //                 Physics => "ფიზიკა",
    //                 Chemistry => "ქიმია",
    //                 Biology => "ბიოლოგია",
    //                 Geography => "გეოგრაფია",
    //             };
    //             all_cor.push((
    //                 format!("{:.1},{}", scaled_score.score, subject),
    //                 format!("{:.1},{}", dp.equalized, subject),
    //             ));
    //             writer
    //                 .write_record([
    //                     dp.equalized.to_string(),
    //                     format!("{:.1}", scaled_score.score),
    //                 ])
    //                 .unwrap();
    //         }

    //         writer.flush().unwrap();
    //     };
    // }

    // let mut writer = Writer::from_path("data/scores-replaced.csv").expect("couldn't open a writer");
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

    //     let score_string = all_cor
    //         .iter()
    //         .fold(score_string, |s, (f, t)| s.replace(f, t));

    //     record[1] = score_string.as_str();
    //     writer.write_record(record).unwrap();
    // }

    // writer.flush().unwrap();
}
