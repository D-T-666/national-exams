use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

use calamine::{open_workbook, DataType, Reader, Xlsx};
use itertools::Itertools;

use csv::Writer;

#[derive(Debug, Clone)]
struct DataPoint {
    variant: i32,
    raw_score: i32,
    equalized: f32,
    density: i32,
    subject: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut processed_data: Vec<DataPoint> = Vec::new();
    let mut input_data: Xlsx<_> = open_workbook(args[1].as_str()).unwrap();
    let mut subjects = Vec::new();
    for (_range_name, range) in input_data.worksheets() {
        let Some(entry) = range.get((0, 0)) else {
            return;
        };
        let entry: String = match entry {
            DataType::String(val) => val.to_owned(),
            _ => String::new(),
        };
        subjects.push(
            entry.split("(").collect::<Vec<&str>>()[1]
                .split(")")
                .collect::<Vec<&str>>()[0]
                .to_string(),
        );
        for (_row_index, row) in range.rows().enumerate().skip(3) {
            for i in (1..row.len()).step_by(3) {
                processed_data.push(DataPoint {
                    raw_score: match row[0] {
                        DataType::Float(x) => x as i32,
                        _ => 0,
                    },
                    equalized: match row[i + 2] {
                        DataType::Float(x) => x as f32,
                        _ => 0.0,
                    },
                    density: match row[i] {
                        DataType::Float(x) => x as i32,
                        _ => 0,
                    },
                    subject: {
                        // let Some(x) = subjects.last() else { return };
                        // x.to_owned()
                        _range_name.to_owned()
                    },
                    variant: (i as i32 + 2) / 3,
                });
            }
        }
    }
    processed_data.sort_by_key(|dp| (dp.equalized * 100.0) as i32);
    processed_data.sort_by_key(|dp| dp.subject.clone());

    processed_data = processed_data
        .iter()
        .peekable()
        .batching(|it| {
            let Some(dp) = it.next() else { return None };
            let dp = dp.to_owned();

            let Some(val) = it.peek() else {
                return Some(dp);
            };

            if val.subject != dp.subject || ((val.equalized - dp.equalized) as f32).abs() >= 0.05 {
                return Some(dp);
            }

            let val = val.to_owned();
            it.next();
            Some(DataPoint {
                variant: -1,
                raw_score: if val.raw_score == dp.raw_score {
                    val.raw_score
                } else {
                    -1
                },
                equalized: val.equalized,
                density: val.density + dp.density,
                subject: val.subject.clone(),
            })
        })
        .collect();

    let mut writers = HashMap::new();

    for dp in processed_data {
        if !writers.contains_key(&dp.subject) {
            if !Path::new("data").is_dir() {
                fs::create_dir("data").unwrap();
            }
            writers.insert(
                dp.subject.clone(),
                Writer::from_path(format!("data/{}.csv", dp.subject.clone())).unwrap(),
            );
        }
        let Some(wrt) = writers.get_mut(&dp.subject) else {
            return;
        };
        wrt.write_record([
            format!("{}", dp.variant),
            format!("{}", dp.density),
            format!("{}", dp.raw_score),
            format!("{:.2}", dp.equalized),
        ])
        .unwrap();
    }

    for wrt in writers.values_mut() {
        wrt.flush().unwrap();
    }
}
