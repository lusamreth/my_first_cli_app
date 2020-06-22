use super::template_engine;
use super::utility;
use super::{gen_newline, gen_whitespace, text_processing, TerminalSize};
use std::collections::VecDeque;
pub enum BorderWeight {
    Normal,
    Bold,
    Bolder,
    Light,
}
fn long_str(content: &Vec<String>) -> usize {
    let mut capture_index = 0;
    let mut max = content.get(0).unwrap();
    for (idx, ct) in content.iter().enumerate() {
        if ct.len() > max.len() {
            max = ct;
            capture_index = idx;
        }
    }
    return capture_index;
}

trait Weighta {
    fn build_weight_justifier(&self);
}
struct Border<'a> {
    filler: &'a str,
    weight: BorderWeight,
    length: usize,
}
type template = Vec<String>;

type Painter<'a> = Box<dyn Fn(template) + 'a>;
impl<'a> Border<'a> {
    fn new(w: BorderWeight) -> Self {
        Border {
            filler: "",
            weight: w,
            length: 0,
        }
    }
    fn len(mut self, s: usize) -> Self {
        self.length = s;
        self
    }
    fn fill(mut self, s: &'a str) -> Self {
        self.filler = s;
        self
    }
    fn draw_vertical(&self, full_len: usize) -> String {
        let mut line = String::new();
        for _ in 0..self.length + 1 {
            line.push_str(self.filler);
        }
        if line.len() > self.length {
            let mut filler_crump = text_processing::CrumpCluster::break_chunk(&line);
            filler_crump.delete(full_len, Some(filler_crump.len()));
            line = filler_crump.merge_crump();
        }
        return line;
    }

    fn build_painter<'b>(&self, full_len: usize) -> impl Fn(&mut template) + '_ {
        let dividend: usize;
        match self.weight {
            BorderWeight::Bold => dividend = self.filler.len() * 2,
            BorderWeight::Bolder => dividend = self.filler.len() * 4,
            BorderWeight::Light => dividend = self.filler.len() / 4,
            BorderWeight::Normal => dividend = self.filler.len(),
        };
        let c = move |tp: &mut template| {
            // smallest == 1;
            if self.filler.len() >= dividend {
                for _ in self.filler.chars().skip(dividend) {
                    tp.push(self.draw_vertical(full_len).to_string());
                }
            } else {
                for _ in 0..self.filler.len() * (dividend - 1) {
                    tp.push(self.draw_vertical(full_len).to_string());
                }
            }
        };
        return Box::new(c);
        // return Box::new(hmm);
    }
}

pub fn convert_queue<T>(raw_content: Vec<T>) -> VecDeque<T> {
    return raw_content.into_iter().collect::<VecDeque<T>>();
}

pub fn border(
    style: &str,
    mut content: template,
    weight: BorderWeight,
    padding: template_engine::Padding,
) -> template {
    let mut rendered_template = Vec::new();
    let longest_element = long_str(&content);
    let len_cont = content
        .get(longest_element)
        .unwrap_or(&"".to_string())
        .len() as i32;
    // let len_cont = (TerminalSize::retrieve().x - 30) as i32;
    content.iter().for_each(|x| println!("{}",x.len()));
    //convert raw content into queue buffer
    let padding_str = padding
        .into_iter()
        .map(|num_gen| gen_whitespace(num_gen as i32))
        .collect::<Vec<String>>();

    if len_cont == 0 {
        content.push("    ".to_string());
    }
    content.insert(content.len(), "".to_string());
    content.insert(0, "".to_string());
    content.iter_mut().enumerate().for_each(|(_, line)| {
        let cal_index = (len_cont as i32 - line.len() as i32 ) as i32;
        let newspace = gen_whitespace(cal_index);
        let mut line_queue = convert_queue(line.chars().into_iter().collect());
        format!("{}{}{}", padding_str[0], newspace, style)
            .chars()
            .into_iter()
            .for_each(|letter| line_queue.push_back(letter));

        format!("{}{}", padding_str[1], style)
            .chars()
            .into_iter()
            .for_each(|z| line_queue.push_front(z));
        
        *line = line_queue.into_iter().collect::<String>();
        // line.push_str(&formated);
    });
    let paint_config = Border::new(weight).fill(style).len(len_cont as usize);

    let drawborder = paint_config.build_painter(content.get(0).unwrap().len() );

    drawborder(&mut rendered_template);
    content.into_iter().for_each(|x| rendered_template.push(x));
    drawborder(&mut rendered_template);

    return rendered_template;
}

pub fn parse_in_template(content: &str) -> Vec<String> {
    let len = 0;
    // dot.write_x();
    let mut render_temp: template = Vec::new();
    let print_preset = |line: &str, count: i32| {
        let formated_str = format!("{}|   {}", count, line);
        return format!("{}", formated_str);
    };
    let mut count_line = 0;
    for line in content.lines() {
        // let line = line.trim();
        if line.len() > len as usize {
            let term_size = TerminalSize::retrieve().x  / 2;
            let muti_lines = utility::split_chunk(line, term_size as usize);
            for line in muti_lines {
                render_temp.push(print_preset(&line, count_line));
                count_line += 1;
            }
        } else {
            render_temp.push(print_preset(line, count_line));
            count_line += 1;
        }
    }
    return render_temp;
}

pub fn center_box(x: i32, y: i32, mut content: template) -> template {
    let mut rendered_template = Vec::new();
    let longest = long_str(&content);
    let len_cont = content.get(longest).unwrap().len() as i32;
    let t_size = x as i32;
    let numx = (t_size - len_cont) / 2;
    let each_space = gen_whitespace(numx - 1);
    rendered_template.push(gen_newline(y));
    content.iter_mut().for_each(|line| {
        let cal_index = (len_cont as i32 - line.len() as i32) as i32;
        let newspace = gen_whitespace(cal_index);
        let formated = format!("{}", newspace);
        line.push_str(&formated);
    });
    content
        .into_iter()
        .for_each(|x| rendered_template.push(format!("{}{}{}", each_space, x, each_space)));
    rendered_template.push(gen_newline(y));
    return rendered_template;
}
//54|   nice but wouldn’t be Vec::with_capacity(string.len() / sub_len) better here?
//54|   nice but wouldn’t be Vec::with_capacity(string.len() / sub_len) better here?
