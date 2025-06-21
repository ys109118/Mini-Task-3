use printpdf::*;
use std::fs::File;
use std::io::{self, BufWriter, Write};

struct Student {
    name: String,
    total_marks: f32,
    num_subjects: u32,
}

impl Student {
    fn average(&self) -> f32 {
        self.total_marks / self.num_subjects as f32
    }

    fn grade(&self) -> char {
        let avg = self.average();
        if avg >= 90.0 {
            'A'
        } else if avg >= 75.0 {
            'B'
        } else if avg >= 60.0 {
            'C'
        } else {
            'D'
        }
    }
}

fn create_pdf(student: &Student) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let report = format!(
        "Student Report Card\n\nName: {}\nTotal Marks: {}\nSubjects: {}\nAverage: {:.2}\nGrade: {}",
        student.name,
        student.total_marks,
        student.num_subjects,
        student.average(),
        student.grade()
    );

    current_layer.use_text(report, 12.0, Mm(10.0), Mm(270.0), &font);

    let file = File::create("report_card.pdf").unwrap();
    doc.save(&mut BufWriter::new(file)).unwrap();
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensures prompt shows before input
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // User input
    let name = get_input("Enter student name: ");
    let total_marks: f32 = get_input("Enter total marks: ")
        .parse()
        .expect("Please enter a valid number");
    let num_subjects: u32 = get_input("Enter number of subjects: ")
        .parse()
        .expect("Please enter a valid number");

    let student = Student {
        name,
        total_marks,
        num_subjects,
    };

    println!("Generating report for {}...", student.name);
    create_pdf(&student);
    println!("✅ PDF generated as 'report_card.pdf'");
}
