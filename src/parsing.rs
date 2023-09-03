use std::{cmp::Ordering, hash::Hash};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Subject {
    Math,
    History,
    Physics,
    Biology,
    Chemistry,
    Geography,
    Literature,
    English,
    Georgian,
}

pub const ALL_SUBJECTS: [Subject; 9] = [
    Subject::Math,
    Subject::History,
    Subject::Physics,
    Subject::Biology,
    Subject::Chemistry,
    Subject::Geography,
    Subject::Literature,
    Subject::English,
    Subject::Georgian,
];

impl ToString for Subject {
    fn to_string(&self) -> String {
        use Subject as S;
        String::from(match self {
            S::Georgian => "ქართული",
            S::English => "უცხოური",
            S::Math => "მათემატიკა",
            S::History => "ისტორია",
            S::Physics => "ფიზიკა",
            S::Chemistry => "ქიმია",
            S::Biology => "ბიოლოგია",
            S::Geography => "გეოგრაფია",
            S::Literature => "ლიტერატურა",
        })
    }
}

impl Subject {
    pub fn from(s: &str) -> Option<Self> {
        use Subject as S;
        match s {
            "ქართული" => Some(S::Georgian),
            "უცხოური" => Some(S::English),
            "მათემატიკა" => Some(S::Math),
            "ისტორია" => Some(S::History),
            "ფიზიკა" => Some(S::Physics),
            "ქიმია" => Some(S::Chemistry),
            "ბიოლოგია" => Some(S::Biology),
            "გეოგრაფია" => Some(S::Geography),
            "ლიტერატურა" => Some(S::Literature),
            _ => None,
        }
    }

    pub fn color(&self) -> String {
        use Subject as S;
        String::from(match self {
            S::Georgian => "pink",
            S::English => "blue",
            S::Math => "green",
            S::History => "orange",
            S::Physics => "red",
            S::Chemistry => "purple",
            S::Biology => "violet",
            S::Geography => "cyan",
            S::Literature => "yellow",
        })
    }
}

impl PartialOrd for Subject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, _) = ALL_SUBJECTS.iter().find_position(|&e| e == self).unwrap();
        let (b, _) = ALL_SUBJECTS.iter().find_position(|&e| e == other).unwrap();

        Some(a.cmp(&b))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScaledScore {
    pub scaled: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct EqualizedScore {
    pub equalized: f32,
    pub scaled: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Score {
    Scaled(f32),
    Equalized(f32),
    EqualizedAndScaled { scaled: f32, equalized: f32 },
}

impl ToString for Score {
    fn to_string(&self) -> String {
        match self {
            Score::Scaled(score) | Score::Equalized(score) => format!("{score:.2}"),
            Score::EqualizedAndScaled { scaled, equalized } => format!("{equalized:.2}-{scaled}"),
        }
    }
}

impl Score {
    pub fn to_latex(&self) -> String {
        match self {
            Score::Scaled(score) => format!("{{\\color{{gray}}\\scriptsize{score:.1}}}"),
            Score::Equalized(score) => format!("{score:.1}"),
            Score::EqualizedAndScaled { scaled, equalized } => format!(
                "{:.1}{{\\color{{gray}}\\scriptsize({scaled:.1})}}",
                (*equalized * 10.0).round() / 10.0
            ),
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct Faculty {
    pub id: String,
    pub name: String,
    pub subjects: [bool; 9],
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct School {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
}

pub const SCHOOLS_SHORT_NAMES_CSV: &str = include_str!("data/schools.csv");

#[derive(Debug, Clone, Copy)]
pub enum Grant {
    Zero,
    Fifty,
    Seventy,
    Hundred,
}

impl ToString for Grant {
    fn to_string(&self) -> String {
        String::from(match self {
            Grant::Zero => "0",
            Grant::Fifty => "50",
            Grant::Seventy => "70",
            Grant::Hundred => "100",
        })
    }
}

#[derive(Debug, Clone)]
pub struct StudentData {
    pub id: String,
    pub scores: [Option<Score>; 9],
    pub overall_score: String,
    pub placement: Option<usize>,
    pub faculty_id: String,
    pub grant: Option<Grant>,
}

#[derive(Debug, Default, Clone)]
pub struct SubjectStats {
    pub min: Option<Score>,
    pub max: Option<Score>,
    pub anchors: Vec<Score>,
}
