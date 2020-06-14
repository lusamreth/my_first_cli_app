pub use console;
pub use console::Alignment;
use cpx2::file;
use std::io::Write;
use termcolor::{BufferWriter,WriteColor,Color,ColorSpec,ColorChoice};
use cpx2::utility;
use regex::Regex;
pub fn add_two(x: i32) -> i32 {
    x + 2
}
#[cfg(test)]
mod iterator_test {
    #[test]
    fn increment() {
        let test_vec = vec![1, 2, 3];
        let up_num = 2;

        let result: Vec<i32> = test_vec.iter().map(|f| f + up_num).collect();
        let expected = vec![3, 4, 5];
        assert_eq!(expected, result);
    }

    struct CountStk {
        count: i32,
        max: Option<i32>,
    }
    impl CountStk {
        fn new(maximum: Option<i32>) -> CountStk {
            CountStk {
                count: 0,
                max: maximum,
            }
        }
    }
    impl Iterator for CountStk {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            match self.max {
                Some(val) => {
                    if self.count < val {
                        self.count += 1;
                        Some(self.count)
                    } else {
                        None
                    }
                }
                None => {
                    self.count += 1;
                    Some(self.count)
                }
            }
        }
    }
    #[test]
    fn test_newly_made_iter() {
        let stk = vec!["a", "b", "c", "d"];
        let mut cstk = CountStk::new(None);

        // assert_eq!(cstk.next(),Some(1)); true
        // assert_eq!(cstk.next(),Some(2)); true
        // assert_eq!(cstk.next(),Some(3)); true
        /*....*/

        // test stk to count real stk;
        // count according to the vec len

        for n in stk {
            println!("{:#?} {:?}", cstk.next().unwrap(), n);
        }
    }
}
#[test]
fn test_file_module() {
    let test_file = file::Fileconfig::new("file read foo.txt ", utility::getnow).unwrap();
    let res = file::Operation::search(&test_file,"apple");
    // file::Operation::update(&test_file, "fkk fkff", "a b c");
    // let hmmm = test_file.run();
    println!("res! {:#?}", res);

    let aye = console::pad_str("apple orange juice", 50, Alignment::Center, None);
    let tester = "happy buggy bomboxi ahhhs ah aaasdh ";
}

fn split_chunk(line: &str, limit: usize) -> Vec<&str> {
    pub use std::str;
    line.as_bytes()
        .chunks(limit)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

#[test]
fn regex_test(){
    let test_regex = regex::Regex::new(r"[a-zA-Z]").unwrap();
    let test_str = "1231 abj hksd";

    match test_regex.captures(test_str){
        Some(var) => println!("{:?}",var),
        None => println!("none")
    };
    
    let a = std::rc::Rc::new(300);
    let b = &a;
    fn tatat(st:i32){
        println!("In function {}",st);
    }
    tatat(*a);
    {
        let z = &b;
    }
    println!("vv{}",b);
    let c = &a;
 }
#[test]
fn writeSimple() -> std::io::Result<()>{
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(buffer,"this is reddddddd")?;
    bufwtr.print(&buffer)
}