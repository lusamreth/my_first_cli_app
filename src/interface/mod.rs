use super::utility;
// use std::cell::RefCell;
// use std::collections::VecDeque;
// use std::rc::Rc;
use terminal_size::{terminal_size, Height, Width};
pub mod template_engine;
pub mod text_processing;
pub mod printer;
pub mod components;
#[derive(Debug)]
pub struct TerminalSize {
    pub x: u16,
    pub y: u16,
}
impl TerminalSize {
    pub fn create(x: u16, y: u16) -> Self {
        return TerminalSize { x, y };
    }
    pub fn retrieve() -> Self {
        let new_size = terminal_size();
        match new_size {
            Some((Width(cli_width), Height(cli_height))) => {
                return TerminalSize::create(cli_width, cli_height);
            }
            None => {
                panic!("Unable to get terminal Sizes!");
            }
        }
    }
}
pub fn gen_whitespace(width: i32) -> String {
    let mut count = 0;
    let mut string_result = String::new();
    while count < width {
        string_result.push_str(" ");
        count += 1;
    }
    return string_result;
}
pub fn gen_newline(height: i32) -> std::string::String {
    let mut count = 0;
    let mut string_result = String::new();
    while count < height {
        string_result.push_str("\n");
        count += 1;
    }
    return string_result;
}
type template = Vec<String>;
pub fn rectangle_box(x: i32, y: i32, content: &str) -> template {
    let mut rendered_template = Vec::new();

    let len_cont = content.len() as i32;
    let t_size = x as i32;

    let numx = (t_size - len_cont) / 2;
    let each_space = gen_whitespace(numx);
    let zo = y / 2;
    rendered_template.push(gen_newline(zo));
    let render = format!("{}{}{}", each_space, content, each_space);
    rendered_template.push(render);
    rendered_template.push(gen_newline(zo));

    return rendered_template;
}
//center
fn recto(x: i32, y: i32, mut content: template) -> template {
    let mut rendered_template = Vec::new();
    let longest = long_str(&content);
    let len_cont = content.get(longest).unwrap().len() as i32;
    let t_size = x as i32;
    let numx = (t_size - len_cont) / 2;
    println!("{}", len_cont);
    let each_space = gen_whitespace(numx - 1);
    content.iter_mut().for_each(|line| {
        let cal_index = (len_cont as i32 - line.len() as i32) as i32;
        let newspace = gen_whitespace(cal_index);
        let formated = format!("{}+", newspace);
        line.push_str(&formated);
    });
    content
        .into_iter()
        .for_each(|x| rendered_template.push(format!("{}{}{}", each_space, x, each_space)));

    return rendered_template;
}
fn long_str(content: &Vec<String>) -> usize {
    let mut capture_index = 0;
    let max = content.get(0);

    match max {
        Some(mut max_val) => {
            for (idx, ct) in content.iter().enumerate() {
                if ct.len() > max_val.len() {
                    max_val = ct;
                    capture_index = idx;
                }
            }
        }
        None => capture_index = 0,
    }

    return capture_index;
}
pub fn parse_in_template(content: &str) -> template {
    let len = 0;
    // dot.write_x();
    let mut render_temp: template = Vec::new();
    let print_preset = |line: &str, count: i32| {
        let formated_str = format!("{}|   {}", count, line);
        return format!("{}", formated_str);
    };
    let mut count_line = 0;
    for line in content.lines() {
        let line = line.trim();
        if line.len() > len as usize {
            let muti_lines = utility::split_chunk(line, 100);
            for line in muti_lines {
                render_temp.push(print_preset(&line, count_line));
                count_line += 1;
            }
        } else {
            render_temp.push(print_preset(&line, count_line));
            count_line += 1;
        }
    }
    let longest_len = long_str(&render_temp) as i32;

    println!("newspace {:?}", render_temp);
    return render_temp;
}



#[test]
fn test() {
    let content = "apsndjakddajdjadb";
    let len_cont = content.len() as i32;
    let t_size = TerminalSize::retrieve().x as i32;

    let numx = (t_size - len_cont) / 2;
    let each_space = gen_whitespace(numx);
    print!("{}", gen_newline(5));
    for _ in 0..10 {
        println!("{}{}{}", each_space, content, each_space);
    }
}

#[test]
fn test_highlighter() {
    let highlight = |x: &str| {
        let style = console::Style::new().on_red();
        let new = style.apply_to(x.to_string());
        return new;
    };
    let matcher = regex::Regex::new(r"\?").unwrap();
    let content = "bombomb?bombomobb sd\"as2311\" asdas asd?as a?sda da?sd as?d";

    let style = console::Style::new().green();
    let stylist_content = style.apply_to(content);
    let new = content
        .to_string()
        .replace("s", &highlight("x").to_string());
    matcher.replace_all(&stylist_content.to_string(), "apple");
    println!("content: {}", new);
    let new_one = regex::Regex::new(r#""[a-zA-Z]+\d+""#).unwrap();

    println!("quote \" {:?}", new_one.find(content));
}
