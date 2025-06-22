use std::io;
use printpdf::*;
use chrono::Local;

#[derive(Debug)]
struct Student {
    name: String,
    total_marks: f64,
    num_subjects: u32,
}

impl Student {
    fn new(name: String, total_marks: f64, num_subjects: u32) -> Self {
        Student { name, total_marks, num_subjects }
    }

    fn average(&self) -> f64 {
        self.total_marks / self.num_subjects as f64
    }

    fn grade(&self) -> String {
        match self.average() {
            avg if avg >= 90.0 => "A".to_string(),
            avg if avg >= 75.0 => "B".to_string(),
            avg if avg >= 60.0 => "C".to_string(),
            _ => "D".to_string(),
        }
    }
}

fn main() {
    println!("📘 Student Report Card Generator\n");

    print!("Enter Student Name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let total_marks = loop {
        print!("Enter Total Marks: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(n) => break n,
            _ => println!("❌ Invalid input. Try again."),
        }
    };

    let num_subjects = loop {
        print!("Enter Number of Subjects: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(n) if n > 0 => break n,
            _ => println!("❌ Must be a positive number."),
        }
    };

    let student = Student::new(name.clone(), total_marks, num_subjects);
    let average = student.average();
    let grade = student.grade();

    println!("\n✅ Report Card Generated\n");
    println!("Name             : {}", student.name);
    println!("Total Marks      : {:.2}", student.total_marks);
    println!("Subjects         : {}", student.num_subjects);
    println!("Average          : {:.2}", average);
    println!("Grade            : {}", grade);

    match generate_pdf(&student, average, &grade) {
        Ok(_) => println!("\n📄 PDF saved as: {}.pdf", student.name.replace(" ", "_")),
        Err(e) => eprintln!("❌ PDF generation failed: {}", e),
    }
}

fn generate_pdf(student: &Student, avg: f64, grade: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Report", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(std::fs::File::open("/home/saikesh/rust/student_report_card/font/LuxuriousRoman-Regular.ttf")?)?;

    let info = vec![
        format!("Name            : {}", student.name),
        format!("Total Marks     : {:.2}", student.total_marks),
        format!("Subjects        : {}", student.num_subjects),
        format!("Average         : {:.2}", avg),
        format!("Grade           : {}", grade),
        format!("Generated On    : {}", Local::now().format("%Y-%m-%d %H:%M:%S")),
    ];

    let title = "🎓 Student Report Card";
    layer.use_text(title, 32.0, Mm(35.0), Mm(250.0), &font);

    let mut y = 200.0;
    for line in info {
        layer.use_text(line, 18.0, Mm(40.0), Mm(y), &font);
        y -= 20.0;
    }

    layer.use_text("Signature ____________________", 14.0, Mm(40.0), Mm(40.0), &font);

    let file_name = format!("{}.pdf", student.name.replace(" ", "_"));
    doc.save(&mut std::io::BufWriter::new(std::fs::File::create(file_name)?))?;
    Ok(())
}
