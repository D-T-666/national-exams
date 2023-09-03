use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

// use cpython::{PyResult, Python};
use csv::{ReaderBuilder, StringRecord};
use gnuplot::AutoOption::Fix;
use gnuplot::PlotOption::{Caption, Color};
use gnuplot::{AxesCommon, Figure};
use itertools::Itertools;

// mod lib;
mod parsing;
mod processing;
mod read;
use parsing::*;
use processing::*;
use read::*;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn save_faculty_plot(
    students: &[StudentData],
    faculty: &Faculty,
    name: &str,
) -> Result<(), gnuplot::GnuplotInitError> {
    let subjects = faculty
        .subjects
        .iter()
        .enumerate()
        .filter_map(|(i, &s)| if s { Some(ALL_SUBJECTS[i]) } else { None })
        .rev()
        .collect_vec();

    let mut fg = Figure::new();

    fg.set_multiplot_layout(3, 2);
    {
        let mut students = students.iter().collect_vec();

        for subject in &subjects {
            // let subject = subjects.iter().fold(&subjects[0], |a, students| {
            //     ALL_SUBJECTS
            //         .iter()
            //         .find(|&x| x == a || x == students)
            //         .unwrap()
            // });

            students.sort_by(|a, b| {
                let a = match a.scores[*subject as usize].unwrap() {
                    Score::Scaled(scaled) => scaled,
                    Score::EqualizedAndScaled {
                        scaled,
                        equalized: _,
                    } => scaled,
                    Score::Equalized(equalized) => equalized,
                };
                let b = match b.scores[*subject as usize].unwrap() {
                    Score::Scaled(scaled) => scaled,
                    Score::EqualizedAndScaled {
                        scaled,
                        equalized: _,
                    } => scaled,
                    Score::Equalized(equalized) => equalized,
                };

                f32::partial_cmp(&b, &a).unwrap()
            });

            let axes2d = fg
                .axes2d()
                .set_x_range(Fix(0.0), Fix(students.len() as f64 + 1.0));

            for subject in &subjects {
                let color = subject.color();
                let subject_string = subject.to_string();
                axes2d.lines_points(
                    1..=students.len(),
                    students
                        .iter()
                        .map(|sd| match sd.scores[*subject as usize].unwrap() {
                            Score::Scaled(scaled) => scaled,
                            Score::EqualizedAndScaled {
                                scaled: _,
                                equalized,
                            } => equalized,
                            Score::Equalized(equalized) => equalized,
                        }),
                    &[Caption(subject_string.as_str()), Color(color.as_str())],
                );
            }
        }
    }
    let axes2d = fg.axes2d();

    axes2d
        .lines_points(
            1..=students.len(),
            students
                .iter()
                .map(|sd| sd.overall_score.parse::<f32>().unwrap()),
            &[Caption("საკონკურსო"), Color("black")],
        )
        .set_x_range(Fix(0.0), Fix(students.len() as f64 + 1.0));

    fg.save_to_eps(&name, 6.0, 8.0)
}

fn compile_pdf(book_path: &str) {
    println!("compiling latex... ");
    Command::new("xelatex")
        .current_dir(Path::new(book_path))
        .arg("main.tex")
        .output()
        .expect("couldn't compile latex");
    println!("done.");
}

struct PDFMaker {
    work_path: String,
    output_file: String,
    top_list_string: Option<String>,
    faculty_strings: Option<Vec<(String, String)>>,
    has_graphs: HashSet<String>,
}

impl PDFMaker {
    fn new(work_path: String, output_file: String) -> Self {
        if !Path::new(work_path.as_str()).exists() {
            fs::create_dir(work_path.as_str()).unwrap();
        }

        let chapter_forlder = format!("{}/chapters", work_path.as_str());
        if !Path::new(chapter_forlder.as_str()).exists() {
            fs::create_dir(&chapter_forlder).unwrap();
        }

        Self {
            work_path: work_path,
            output_file: output_file,
            top_list_string: None,
            faculty_strings: None,
            has_graphs: HashSet::new(),
        }
    }

    fn compile(&mut self) -> &mut Self {
        compile_pdf(self.work_path.as_str());

        fs::rename(
            format!("{}/main.pdf", self.work_path),
            format!("{}.pdf", self.output_file),
        )
        .unwrap();
        fs::remove_dir_all(self.work_path.as_str()).unwrap();

        self
    }

    fn save(&mut self) -> io::Result<&mut Self> {
        let mut main_file_inputs = Vec::new();

        if let Some(top_list) = &self.top_list_string {
            main_file_inputs.push(String::from("\\input{top-list}"));

            let mut writer = File::create(format!("{}/top-list.tex", self.work_path))?;
            writer.write_all(top_list.as_bytes())?;
        }

        if let Some(faculty_strings) = &self.faculty_strings {
            for (faculty_id, faculty_string) in faculty_strings {
                let mut faculty_string = String::from(faculty_string);

                main_file_inputs.push(format!("\\input{{chapters/{}}}", faculty_id));

                let mut writer =
                    File::create(format!("{}/chapters/{}.tex", self.work_path, faculty_id))?;

                if self.has_graphs.contains(faculty_id) {
                    faculty_string += format!(
                        "\n\\begin{{figure}}[H]\\centering
    \\includegraphics{{chapters/{faculty_id}.eps}}
\\end{{figure}}"
                    )
                    .as_str();
                }

                writer.write_all(faculty_string.as_bytes())?;
            }
        }

        let main_file = format!(
            "\\documentclass{{article}}

\\usepackage[margin=2cm]{{geometry}}

\\usepackage{{fontspec}}
\\usepackage{{float}}
\\usepackage{{graphics}}
\\usepackage{{xcolor}}

\\usepackage[T1]{{fontenc}}
\\setmainfont{{GA Sylvia}}
\\usepackage[georgian]{{babel}}
\\usepackage{{longtable,array}}

\\newcolumntype{{C}}[1]{{>{{\\centering\\arraybackslash}}p{{#1}}}}

\\begin{{document}}
\t{}
\\end{{document}}",
            if !main_file_inputs.is_empty() {
                main_file_inputs.join("\n\t")
            } else {
                String::from("no data")
            }
        );

        let mut book_writer = File::create(format!("{}/main.tex", self.work_path))
            .expect("couldn't create book file");
        book_writer.write_all(main_file.as_bytes())?;

        Ok(self)
    }

    fn write_graphs(
        &mut self,
        faculty_buckets: &Vec<(&String, &[StudentData])>,
        faculties: &HashMap<String, Faculty>,
    ) -> &mut Self {
        println!("writing graphs... ");

        for (faculty_id, students) in faculty_buckets {
            if students.len() > 1 {
                self.has_graphs.insert(String::from(faculty_id.to_owned()));
            }
        }

        faculty_buckets
            .par_iter()
            // .iter()
            .for_each(|(faculty_id, students)| {
                if students.len() > 1 {
                    save_faculty_plot(
                        students,
                        faculties.get(*faculty_id).unwrap(),
                        format!("{}/chapters/{faculty_id}.eps", self.work_path).as_str(),
                    )
                    .unwrap();
                }
            });

        println!("done.");
        self
    }

    fn write_faculties(
        &mut self,
        faculty_buckets: &Vec<(&String, &[StudentData])>,
        schools: &HashMap<String, School>,
        faculties: &HashMap<String, Faculty>,
    ) -> &mut Self {
        let mut full_res = Vec::new();
        for (faculty_id, students) in faculty_buckets {
            let faculty_id = String::from(*faculty_id);
            let faculty = faculties.get(&faculty_id).unwrap();
            let school = schools.get(&faculty_id[0..3]).unwrap();

            let mut res = String::new();

            let subjects = faculty
                .subjects
                .iter()
                .enumerate()
                .filter_map(|(i, &s)| if s { Some(ALL_SUBJECTS[i]) } else { None })
                .rev()
                .collect_vec();

            res += format!(
                "\\section*{{{} - {}}}\n\\subsection*{{{}}}\n",
                faculty.id,
                school.name.trim(),
                faculty.name.trim()
            )
            .as_str();

            res += format!(
                    "\n\\begin{{longtable}}{{ C{{0.03\\textwidth}} C{{0.07\\textwidth}} C{{0.08\\textwidth}}{}C{{0.07\\textwidth}}}}",
                    " C{0.1\\textwidth} ".repeat(2 + subjects.len())
                )
                .as_str();

            res += format!(
                "\n\t{} \\\\\\hline",
                [
                    vec![
                        String::from(""),
                        String::from("ადგილი"),
                        String::from("ნომერი")
                    ],
                    subjects.iter().map(|a| a.to_string()).collect_vec(),
                    vec![String::from("საკონკურსო"), String::from("გრანტი")]
                ]
                .concat()
                .join(" & ")
            )
            .as_str();

            for (student_index, student_data) in students.iter().enumerate() {
                let scores = subjects
                    .iter()
                    .map(|subject| student_data.scores[*subject as usize].unwrap().to_latex())
                    .collect_vec();

                res += format!(
                    "\n\t{} \\\\",
                    [
                        vec![
                            format!("\\color{{gray}}{}", student_index + 1),
                            student_data.placement.unwrap().to_string(),
                            format!("\\color{{gray}}{}", student_data.id)
                        ],
                        scores,
                        vec![
                            student_data.overall_score.clone(),
                            match student_data.grant {
                                Some(x) => x.to_string(),
                                None => String::from(""),
                            }
                        ]
                    ]
                    .concat()
                    .join(" & ")
                )
                .as_str();
            }

            res += "\n\\end{longtable}";

            full_res.push((String::from(faculty_id), res));
        }

        self.faculty_strings = Some(full_res);

        self
    }

    fn write_top_list(
        &mut self,
        students: &[StudentData],
        schools: &HashMap<String, School>,
        faculties: &HashMap<String, Faculty>,
    ) -> &mut Self {
        let mut res = String::new();

        res += "\\section*{აბიტურიენტები საკონკურსო ქულის მიხედვით კლებადობით}

{
\\scriptsize
\\begin{longtable}{C{0.04\\textwidth} | C{0.07\\textwidth} | C{0.07\\textwidth} | C{0.08\\textwidth} | C{0.08\\textwidth} | C{0.08\\textwidth} | C{0.35\\textwidth} | C{0.06\\textwidth}}
    \\# & საგანი 1 & საგანი 2 & საგანი 3 & საგანი 4 & საკონკურსო & ფაკულტეტი & გრანტი \\\\ \\hline\\hline";

        for (student_index, student) in students.iter().enumerate() {
            let faculty = faculties.get(&student.faculty_id).unwrap();
            let school = schools.get(&student.faculty_id[0..3]).unwrap();

            let subjects = faculty
                .subjects
                .iter()
                .enumerate()
                .filter_map(|(i, &s)| if s { Some(ALL_SUBJECTS[i]) } else { None })
                .rev()
                .collect_vec();

            let student_line = format!("\t & \\color{{gray}}{} & \\color{{gray}}{} & \\color{{gray}}{} & \\color{{gray}}{} & & \\color{{gray}}{} & \\\\\n\t{} & {} & {} & {} & {} & {} & {} & {}\\\\\\hline\n",
                    match subjects.get(0) {Some(subject) => subject.to_string(), None => String::default()},
                    match subjects.get(1) {Some(subject) => subject.to_string(), None => String::default()},
                    match subjects.get(2) {Some(subject) => subject.to_string(), None => String::default()},
                    match subjects.get(3) {Some(subject) => subject.to_string(), None => String::default()},
                    school.short_name.clone().unwrap_or(school.name.clone()),
                    student_index + 1,
                    match subjects.get(0) {Some(subject) => student.scores[*subject as usize].unwrap().to_latex(), None => String::default()},
                    match subjects.get(1) {Some(subject) => student.scores[*subject as usize].unwrap().to_latex(), None => String::default()},
                    match subjects.get(2) {Some(subject) => student.scores[*subject as usize].unwrap().to_latex(), None => String::default()},
                    match subjects.get(3) {Some(subject) => student.scores[*subject as usize].unwrap().to_latex(), None => String::default()},
                    student.overall_score,
                    faculty.name,
                    student.grant.unwrap_or(parsing::Grant::Zero).to_string()
                );

            res += student_line.as_str();
        }
        res += "\\end{longtable}\n}";

        self.top_list_string = Some(res);

        self
    }
}

//

use clap::Parser;

/// გადააქციე ჩარიცხვებისა და რანჟირებული ქულების
/// PDF ფაილი დესკალირებული და დახარისხებული სიად
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ჩარიცხვების PDF ფაილი
    input_file: String,
    /// დესკალირების მონაცემების CSV ფაილი
    descaling_data_file: String,
    /// დროებითი ფაილების საქაღალდე
    work_path: Option<String>,
    /// შეიცავდეს გრაფიკებს
    #[arg(short, long)]
    graphs: bool,
    /// შეიცავდეს საკონკურსო ქულის მიხედვით 
    /// დახარისხებულ სიას
    #[arg(short, long)]
    top_list: bool,
    /// შეიცავდეს ფაკულტეტებს
    #[arg(short, long)]
    faculties: bool,
    /// შეამოკლოს უნივერსიტეტების სახელები
    #[arg(short, long)]
    shorten_names: bool,
}

fn main() {
    let Cli {
        input_file,
        descaling_data_file,
        work_path,
        graphs,
        top_list,
        faculties,
        shorten_names,
    } = Cli::parse();
    let include_faculties = faculties;
    let work_path = work_path.unwrap_or(input_file.replace(".pdf", "-work-directory"));

    let publication_tsv_file_name = input_file.replace("pdf", "tsv");

    parse_publication_pdf(&input_file, &publication_tsv_file_name);

    let (students, mut schools, faculties) =
        read_publication_tsv(publication_tsv_file_name.as_str());

    fs::remove_file(publication_tsv_file_name).unwrap();

    let students = sort_students(descale_with_independent_data(
        students,
        read_independent_descaling_data(&descaling_data_file),
    ));

    if shorten_names {
        let school_short_names: HashMap<String, String> = {
            let mut map = HashMap::new();

            let mut reader = ReaderBuilder::new().from_reader(SCHOOLS_SHORT_NAMES_CSV.as_bytes());

            while !reader.is_done() {
                let mut csv_line = StringRecord::new();
                reader
                    .read_record(&mut csv_line)
                    .expect("error while reading independent data");

                if csv_line.len() != 2 {
                    continue;
                }

                let csv_line = csv_line.iter().collect_vec();

                map.insert(String::from(csv_line[0]), String::from(csv_line[1]));
            }

            map
        };

        for (id, short_name) in school_short_names {
            if let Some(school) = schools.get_mut(&id.clone()) {
                school.short_name = Some(short_name);
            }
        }
    }

    let faculty_buckets = collect_faculties(students.clone());
    let faculty_buckets = faculty_buckets
        .iter()
        .sorted_by(|a, b| {
            f32::partial_cmp(&a.0.parse::<f32>().unwrap(), &b.0.parse::<f32>().unwrap()).unwrap()
        })
        .map(|(id, students)| (id, students.get(0..students.len()).unwrap()))
        .collect_vec();

    // WRITE OUT

    let work_path = work_path;

    // Compile the PDF

    let mut pdf_out = &mut PDFMaker::new(
        work_path,
        input_file.replace(
            ".pdf",
            [
                Some("-out"),
                if top_list || include_faculties {
                    Some("descaled")
                } else {
                    None
                },
                if top_list { Some("top-list") } else { None },
                if top_list && shorten_names { Some("with-shortened-names") } else { None },
                if top_list && include_faculties {
                    Some("and")
                } else {
                    None
                },
                if include_faculties {
                    Some("faculties")
                } else {
                    None
                },
                if include_faculties && graphs {
                    Some("with-graphs")
                } else {
                    None
                },
            ]
            .iter()
            .filter_map(|&a| a)
            .join("-")
            .as_str(),
        ),
    );

    if top_list {
        pdf_out = pdf_out.write_top_list(&students[..], &schools, &faculties);
    }

    if include_faculties {
        if graphs {
            pdf_out = pdf_out.write_graphs(&faculty_buckets, &faculties);
        }

        pdf_out = pdf_out.write_faculties(&faculty_buckets, &schools, &faculties);
    }

    pdf_out.save().unwrap().compile();
}
