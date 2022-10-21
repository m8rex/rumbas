use rumbas::exam::diagnostic::Diagnostic;
use rumbas::exam::ExamFileType;
use rumbas::support::input_string::InputString;
use structdoc::StructDoc;

fn main() {
    println!("{}", ExamFileType::document().markdown());
    //println!("{}", Diagnostic::document().markdown());
}
