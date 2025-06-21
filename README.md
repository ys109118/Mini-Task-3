
# 📚 Student Report Card – Rust Console App

This Rust-based console application takes student details from the user (name, total marks, and number of subjects), calculates the average, assigns a grade, and generates a clean **PDF report card**.



## ✨ Features

- Accepts student input via console
- Calculates average marks
- Assigns grades based on average:
  - **A**: 90+
  - **B**: 75–89
  - **C**: 60–74
  - **D**: Below 60
- Generates a neat **PDF report card** using `printpdf`



## 🖥️ Demo (Console Output)


Enter student name: Yash Sharma
Enter total marks: 420
Enter number of subjects: 5
Generating report for Yash Sharma...
✅ PDF generated as 'report_card.pdf'




## 🧠 Code Overview

### 1. `Student` Struct

rust
struct Student {
    name: String,
    total_marks: f32,
    num_subjects: u32,
}


Holds the basic student data.



### 2. `impl Student`

rust
fn average(&self) -> f32 { ... }
fn grade(&self) -> char { ... }


- Calculates average marks
- Assigns a letter grade



### 3. PDF Generator

rust
fn create_pdf(student: &Student) { ... }


- Uses `printpdf` to create a report card with name, total, average, and grade.



### 4. User Input

rust
fn get_input(prompt: &str) -> String { ... }


Reusable function to safely get console input and trim it.



### 5. Main Logic

rust
fn main() {
    // Input, calculate, and generate report
}
## 📦 Dependencies

Add to `Cargo.toml`:

toml
[dependencies]
printpdf = "0.5"


## 📄 Output PDF

Generates a file:


report_card.pdf


Contents:


Student Report Card

Name: Yash Sharma
Total Marks: 420
Subjects: 5
Average: 84.00
Grade: B


## 🚀 How to Run

1. Clone the repo:

bash
git clone https://github.com/ys109118/Mini-Task-3.git
cd Mini-Task-3

2. Build and run:

bash
cargo run


3. After entering details, check for `report_card.pdf` in the project folder.

## 🛠️ Requirements

- Rust installed (`https://www.rust-lang.org/tools/install`)
- PDF viewer to open the result


## 📬 Author

[Yash Sharma](https://github.com/ys109118)

---

## 📝 License

This project is for educational/demo purposes.
