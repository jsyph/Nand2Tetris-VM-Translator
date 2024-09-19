mod parse;
mod generation;

pub use parse::parse_lines;
pub use generation::generate_code;

// pub fn test() {
//     for x in parse_lines(vec![
//         String::from("// This file is part of www.nand2tetris.org"),
//         String::from("// and the book \"The Elements of Computing Systems\""),
//         String::from("// by Nisan and Schocken, MIT Press."),
//         String::from("// File name: projects/7/MemoryAccess/BasicTest/BasicTest.vm"),
//         String::from(""),
//         String::from("// Executes pop and push commands."),
//         String::from(""),
//         String::from("push constant 10"),
//         String::from("pop local 0"),
//         String::from("push constant 21"),
//         String::from("push constant 22"),
//         String::from("pop argument 2"),
//         String::from("pop argument 1"),
//         String::from("push constant 36"),
//         String::from("pop this 6"),
//         String::from("push constant 42"),
//         String::from("push constant 45"),
//         String::from("pop that 5"),
//         String::from("pop that 2"),
//         String::from("push constant 510"),
//         String::from("pop temp 6"),
//         String::from("push local 0"),
//         String::from("push that 5"),
//         String::from("add"),
//         String::from("push argument 1"),
//         String::from("sub"),
//         String::from("push this 6"),
//         String::from("push this 6"),
//         String::from("add"),
//         String::from("sub"),
//         String::from("push temp 6"),
//         String::from("add"),
//     ])
//     .unwrap()
//     {
//         println!("{:?}", x);
//     }
// }
