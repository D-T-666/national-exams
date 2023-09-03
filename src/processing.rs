use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

use crate::parsing::*;

fn map_range(v: &f32, from_min: &f32, from_max: &f32, to_min: &f32, to_max: &f32) -> f32 {
    (v - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}

pub fn descale_with_independent_data(
    students: Vec<StudentData>,
    independent_data: [Option<SubjectStats>; ALL_SUBJECTS.len()],
) -> Vec<StudentData> {
    let mut subject_stats = independent_data.map(Option::unwrap);

    for student in &students {
        for (subject_index, score) in student.scores.iter().enumerate() {
            let Some(score) = score else {
                continue;
            };

            let score = match score {
                Score::Scaled(scaled) => scaled,
                Score::EqualizedAndScaled {
                    scaled,
                    equalized: _,
                } => scaled,
                Score::Equalized(_) => continue,
            };

            subject_stats[subject_index].min = match subject_stats[subject_index].min.unwrap() {
                Score::EqualizedAndScaled { scaled, equalized } => {
                    Some(Score::EqualizedAndScaled {
                        scaled: scaled.min(*score),
                        equalized,
                    })
                }
                Score::Equalized(equalized) => Some(Score::EqualizedAndScaled {
                    scaled: *score,
                    equalized,
                }),
                _ => panic!("bad data"),
            };

            subject_stats[subject_index].max = match subject_stats[subject_index].max.unwrap() {
                Score::EqualizedAndScaled { scaled, equalized } => {
                    Some(Score::EqualizedAndScaled {
                        scaled: scaled.max(*score),
                        equalized,
                    })
                }
                Score::Equalized(equalized) => Some(Score::EqualizedAndScaled {
                    scaled: *score,
                    equalized,
                }),
                _ => panic!("bad data"),
            };
        }
    }

    students
        .iter()
        .map(|x| {
            let mut scores = x.scores;

            for subject_index in 0..ALL_SUBJECTS.len() {
                if let Some(score) = &scores[subject_index] {
                    let scaled_score = match score {
                        Score::Scaled(scaled) => scaled,
                        _ => continue,
                    };

                    let subject_data = &subject_stats[subject_index];

                    let Some(Score::EqualizedAndScaled { scaled, equalized }) = &subject_data.min
                    else {
                        continue;
                    };
                    let mut min_scaled = scaled;
                    let mut min_equalized = equalized;
                    let Some(Score::EqualizedAndScaled { scaled, equalized }) = &subject_data.max
                    else {
                        continue;
                    };
                    let mut max_scaled = scaled;
                    let mut max_equalized = equalized;

                    for anchor in &subject_data.anchors {
                        let Score::EqualizedAndScaled { scaled, equalized } = anchor else {
                            panic!("bad anchor")
                        };

                        if scaled <= scaled_score {
                            min_scaled = scaled;
                            min_equalized = equalized;
                        }
                    }

                    for anchor in &subject_data.anchors {
                        let Score::EqualizedAndScaled { scaled, equalized } = anchor else {
                            panic!("bad anchor")
                        };

                        if scaled_score < scaled {
                            max_scaled = scaled;
                            max_equalized = equalized;
                        }
                    }

                    let equalized_score = map_range(
                        scaled_score,
                        min_scaled,
                        max_scaled,
                        min_equalized,
                        max_equalized,
                    );

                    scores[subject_index] = Some(Score::EqualizedAndScaled {
                        scaled: *scaled_score,
                        equalized: equalized_score,
                    });
                }
            }

            StudentData {
                scores,
                id: x.id.clone(),
                overall_score: x.overall_score.clone(),
                placement: x.placement,
                faculty_id: x.faculty_id.clone(),
                grant: x.grant,
            }
        })
        .collect_vec()
}

pub fn sort_students(students: Vec<StudentData>) -> Vec<StudentData> {
    let mut students = students;

    students.sort_by(|a, b| {
        b.overall_score
            .parse::<f32>()
            .unwrap()
            .partial_cmp(&a.overall_score.parse::<f32>().unwrap())
            .unwrap()
    });

    students
        .iter()
        .enumerate()
        .map(|(i, s)| StudentData {
            scores: s.scores,
            id: s.id.clone(),
            overall_score: s.overall_score.clone(),
            placement: Some(i + 1),
            faculty_id: s.faculty_id.clone(),
            grant: s.grant,
        })
        .collect_vec()
}

pub fn collect_faculties(students: Vec<StudentData>) -> HashMap<String, Vec<StudentData>> {
    let mut faculties = HashMap::new();

    for student in students {
        let id = student.faculty_id.clone();

        if !faculties.contains_key(&id) {
            faculties.insert(id.clone(), vec![student]);
        } else {
            faculties.get_mut(&id).unwrap().push(student);
        }
    }

    faculties
}
