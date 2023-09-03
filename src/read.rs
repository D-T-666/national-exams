use std::{collections::HashMap, fs::File, io::Read};

use csv::{ReaderBuilder, StringRecord};
use itertools::Itertools;

use crate::parsing::*;

pub fn read_publication_tsv(
    file_name: &str,
) -> (
    Vec<StudentData>,
    HashMap<String, School>,
    HashMap<String, Faculty>,
) {
    let mut reader = File::open(file_name).unwrap();
    let mut file_contents = String::new();
    reader
        .read_to_string(&mut file_contents)
        .expect("couldn't read the file");
    let file_contents = file_contents.split('\n');

    let mut students = Vec::new();
    let mut schools = HashMap::new();
    let mut faculties = HashMap::new();

    let mut faculty_name: Option<String> = None;
    let mut faculty_id: Option<String> = None;
    let mut current_subjects: Vec<Subject> = Vec::new();

    for raw_line in file_contents {
        let line = raw_line.trim().split('\t').collect_vec();

        if line.iter().all(|col| col.parse::<f32>().is_ok()) {
            if faculty_id.is_some() {
                let student_id = String::from(line[1]);
                let mut scores = [None; ALL_SUBJECTS.len()];

                for (index, subject) in current_subjects.iter().enumerate() {
                    scores[*subject as usize] =
                        Some(Score::Scaled(line[index + 2].parse().unwrap()));
                }

                let overall_score = String::from(line[current_subjects.len() + 2]);
                let grant = if line.len() < current_subjects.len() + 2 {
                    Some(Grant::Zero)
                } else {
                    match line[line.len() - 1] {
                        "100" => Some(Grant::Hundred),
                        "70" => Some(Grant::Seventy),
                        "50" => Some(Grant::Fifty),
                        _ => None,
                    }
                };

                students.push(StudentData {
                    id: student_id,
                    scores,
                    overall_score,
                    placement: None,
                    faculty_id: faculty_id.clone().unwrap(),
                    grant,
                });
            }

            // break;
        } else if line.contains(&"%") {
            let mut subjects = [false; 9];
            current_subjects.clear();

            for elem in line.iter().take(line.len() - 2).skip(1)  {
                if let Some(subject) = Subject::from(elem) {
                    current_subjects.push(subject);
                    subjects[subject as usize] = true;
                }
            }

            let faculty_id = faculty_id.clone().unwrap();

            let faculty: Faculty = Faculty {
                id: faculty_id.clone(),
                name: faculty_name.clone().unwrap(),
                subjects,
            };

            if !faculties.contains_key(&faculty_id) {
                faculties.insert(faculty_id.clone(), faculty);
            }
        } else if line[0].parse::<f32>().is_ok() {
            if line[0].len() == 3 {
                let id = String::from(line[0]);
                let school = School {
                    id: id.clone(),
                    name: line[1..line.len()].join(" "),
                    short_name: None,
                };

                schools.entry(id).or_insert(school);
            } else {
                faculty_name = Some(line[1..line.len()].join(" "));
                faculty_id = Some(String::from(line[0]));
            }
        }
    }

    (students, schools, faculties)
}

pub fn read_independent_descaling_data(
    file_name: &str,
) -> [Option<SubjectStats>; ALL_SUBJECTS.len()] {
    // READ INDEPENDENT DATA

    let mut independent_descaling_data = [None, None, None, None, None, None, None, None, None];

    let mut csv_reader = ReaderBuilder::new()
        .from_path(file_name)
        .expect("couldn't read the independent data file");

    while !csv_reader.is_done() {
        let mut csv_line = StringRecord::new();
        csv_reader
            .read_record(&mut csv_line)
            .expect("error while reading independent data");

        let csv_line = csv_line.iter().collect_vec();

        if csv_line.is_empty() {
            continue;
        }

        let Some(subject) = Subject::from(csv_line[0]) else {
            panic!("cmon")
        };

        let mut min = None;
        let mut max = None;
        let mut anchor = None;

        match csv_line[1] {
            "maximum" => {
                max = Some(Score::Equalized(
                    csv_line[2].parse().expect("invalid number"),
                ))
            }
            "minimum" => {
                min = Some(Score::Equalized(
                    csv_line[2].parse().expect("invalid number"),
                ))
            }
            "anchor" => {
                anchor = Some(Score::EqualizedAndScaled {
                    equalized: csv_line[2].parse().expect("invalid number"),
                    scaled: csv_line[3].parse().expect("invalid number"),
                })
            }
            _ => todo!(),
        }

        match independent_descaling_data[subject as usize] {
            None => {
                independent_descaling_data[subject as usize] = Some(SubjectStats {
                    min,
                    max,
                    anchors: if let Some(anchor) = anchor {
                        vec![anchor]
                    } else {
                        Vec::new()
                    },
                });
            }
            Some(ref mut stats) => {
                if min.is_some() {
                    stats.min = min
                }
                if max.is_some() {
                    stats.max = max
                }
                if let Some(anchor) = anchor {
                    stats.anchors.push(anchor)
                }
            }
        }
    }

    for ref mut idd in independent_descaling_data.iter_mut().flatten() {
        if idd.min.is_none() {
            if let Some(max) = idd.max {
                match max {
                    Score::Equalized(max) => {
                        idd.min = Some(Score::Equalized(max * 0.2));
                    }
                    Score::EqualizedAndScaled {
                        scaled: _,
                        equalized,
                    } => {
                        idd.min = Some(Score::Equalized(equalized * 0.2));
                    }
                    _ => (),
                }
            }
        }
    }

    independent_descaling_data
}

use cpython::{ObjectProtocol, PyModule, PyResult, Python};

pub fn parse_publication_pdf(input_file: &str, output_file: &str) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let ppp = module_from_str(
        py,
        "parse_publication_pdf",
        include_str!("python/parse_publication_pdf.py"),
    )
    .expect("This tool depends on tabula-py. `python -m pip install tabula-py`");

    ppp.call(py, "parse_publication_pdf", (input_file, output_file), None)
        .unwrap();
}

fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;

    let builtins = cpython::PyModule::import(py, "builtins").unwrap();
    m.dict(py).set_item(py, "__builtins__", &builtins).unwrap();

    // OR
    m.add(py, "__builtins__", py.import("builtins")?)?;
    let m_locals = m.get(py, "__dict__")?.extract(py)?;

    // To avoid multiple import, and to add entry to the cache in `sys.modules`.
    let sys = cpython::PyModule::import(py, "sys").unwrap();
    sys.get(py, "modules")
        .unwrap()
        .set_item(py, name, &m)
        .unwrap();

    // Finally, run the moduke
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}
