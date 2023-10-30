// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

use std::fmt::*;
pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{}",self)
    }
}

impl Display for ReportCard{
    fn fmt(&self, f: &mut Formatter<'_>)->Result {
        let mut alphabetic=String::from("");
        if self.grade >=1.0&& self.grade<1.5{
            alphabetic="F-".to_string();
        }else if self.grade >=1.5&& self.grade<2.0{
            alphabetic="F".to_string();
        }else if self.grade >=2.0&& self.grade<2.5{
            alphabetic="E".to_string();
        }else if self.grade >=2.5&& self.grade<3.0{
            alphabetic="D".to_string();
        }else if self.grade >=3.0&& self.grade<3.5{
            alphabetic="C".to_string();
        }else if self.grade >=4.0&& self.grade<4.5{
            alphabetic="B".to_string();
        }else if self.grade >=4.5&& self.grade<5.0{
            alphabetic="A".to_string();
        }else if self.grade >=5.0&& self.grade<=5.5{
            alphabetic="A+".to_string();
        }
        write!(f,"{} ({}) - achieved a grade of {}",self.student_name,self.student_age,alphabetic);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 5.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
