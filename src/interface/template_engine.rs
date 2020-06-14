use super::text_processing;
use super::{border, gen_newline, gen_whitespace, utility, BorderWeight};
use std::collections::VecDeque;
use std::iter;
use std::slice::IterMut;
trait TemplateEngine<'a> {
    // fn gen_whitespace();
    // fn gen_newline();
    fn init() -> TemplateFactory<'a>;
    fn center_box();
    fn parse_in_template();
}
trait TemplateBuilder<'a> {
    fn init() -> TemplateFactory<'a>;
    fn chain(&mut self) -> &mut Self;
    fn border(&mut self, style: &'a str, weight: BorderWeight) -> &mut Self;
    fn create_movable(&mut self) -> &mut Self;
    fn padding(&mut self, padding: Vec<u16>) -> &mut Self;
    fn collect(self) -> Template<'a>;
}
#[derive(Debug)]
struct TemplateFactory<'a> {
    structure: Vec<String>,
    opt_movable: Option<Vec<VecDeque<char>>>,
    opt_style_dot: Option<&'a str>,
    opt_padding: Padding,
}
// impl
#[derive(Debug, Clone)]
struct Template<'a> {
    structure: Vec<String>,
    movable: Vec<VecDeque<char>>,
    style_dot: &'a str,
    padding: Padding,
}

#[derive(Debug, Clone)]
struct Padding {
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}
struct PaddingInterator<'a> {
    current_state: &'a Padding,
    index: usize,
}
impl<'a> IntoIterator for &'a Padding {
    type Item = u16;
    type IntoIter = PaddingInterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        PaddingInterator {
            current_state: self,
            index: 0,
        }
    }
}
impl<'a> Iterator for PaddingInterator<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.current_state.top,
            1 => self.current_state.bottom,
            2 => self.current_state.left,
            3 => self.current_state.right,
            _ => return None,
        };
        self.index += 1;
        return Some(result);
    }
}

impl Padding {
    pub fn create() -> Padding {
        Padding {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }
    fn insert(&mut self, pad: Vec<u16>) {
        self.top = pad[0];
        self.bottom = pad[1];
        self.left = pad[2];
        self.right = pad[3];
    }
    fn chain(&mut self) -> &mut Self {
        self
    }
    // 1len = all / 2 len = divide into two halves / 4 len 1 for each
    pub fn input(&mut self, input: Vec<u16>) -> &mut Self {
        if input.len() > 4 || input.len() == 3 {
            return self.chain();
        }
        let mut con_vec = Vec::with_capacity(4);
        if input.len() == 1 {
            let mut new_pad = Vec::new();
            let e = input.get(0).unwrap();
            new_pad = self
                .into_iter()
                .map(|mut z| {
                    z = 0;
                    z + e
                })
                .collect::<Vec<u16>>();
            self.insert(new_pad);
        } else if input.len() == 2 {
            let mut i = 0;
            let mid = con_vec.capacity() / 2;
            while i < con_vec.capacity() {
                if i < mid {
                    con_vec.push(*input.get(0).unwrap());
                } else {
                    con_vec.push(*input.get(1).unwrap());
                }
                i += 1;
            }
            self.insert(con_vec);
        } else if input.len() == 4 {
            self.insert(input);
        }
        self.chain()
    }
    fn transform(&mut self, T_fx: Box<dyn Fn(u16) -> u16>) -> &mut Self {
        let mut cache = Vec::new();
        let transformed_map: Vec<u16> = self.into_iter().map(|x| T_fx(x)).collect();

        for pad_val in transformed_map {
            cache.push(pad_val);
        }
        *self = Padding {
            top: cache[0],
            left: cache[1],
            bottom: cache[2],
            right: cache[3],
        };
        self.chain()
    }
    pub fn expand(&mut self, expand_factor: u16) -> &mut Self {
        let factorize = move |param: u16| -> u16 { param * expand_factor };
        self.transform(Box::new(factorize))
    }
    pub fn shrink(&mut self, shrink_factor: u16) -> &mut Self {
        let defactorize = move |param: u16| -> u16 { param / shrink_factor };
        self.transform(Box::new(defactorize))
    }
}
#[test]
fn test_temp_engine() {
    let mut newp = Padding::create();
    newp.input(vec![1, 2]).expand(100).shrink(3);
    println!("Newp {:?}", newp);
    // println!("{:?}");
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

type template = Vec<String>;

fn parse_in_template(content: &str) -> Vec<String> {
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
                render_temp.push(print_preset(line, count_line));
                count_line += 1;
            }
        } else {
            render_temp.push(print_preset(line, count_line));
            count_line += 1;
        }
    }
    let longest_len = long_str(&render_temp) as i32;

    println!("newspace {:?}", render_temp);
    return render_temp;
}
impl<'a> TemplateBuilder<'a> for TemplateFactory<'a> {
    fn init() -> Self {
        TemplateFactory {
            structure: Vec::new(),
            opt_movable: None,
            opt_style_dot: None,
            opt_padding: Padding::create(),
        }
    }
    fn border(&mut self, style: &'a str, weight: BorderWeight) -> &mut Self {
        self.opt_style_dot = Some(style);
        let style = self.opt_style_dot.unwrap();
        // self.structure.push("Just a place holder for sure!".to_string());
        self.structure = border(style.clone(), self.structure.clone(), weight);
        self.chain()
    }
    fn chain(&mut self) -> &mut Self {
        return self;
    }
    fn padding(&mut self, padding: Vec<u16>) -> &mut Self {
        self.opt_padding.input(padding);
        self.chain()
    }
    fn create_movable(&mut self) -> &mut Self {
        let m_obj = self.structure.iter().flat_map(|each_cluster| {
            let crump = text_processing::CrumpCluster::break_chunk(each_cluster);
            return crump
                .get_raw()
                .into_iter()
                .map(|eachcrump| eachcrump.into_iter().collect::<VecDeque<char>>())
                .collect::<Vec<VecDeque<char>>>();
        }).collect::<Vec<VecDeque<char>>>();
        self.opt_movable = Some(m_obj);
        self.chain()
    }
    fn collect(self) -> Template<'a> {
        let mut empty = Vec::new();
        empty.push(VecDeque::new());
        Template {
            movable: self.opt_movable.unwrap_or(empty),
            structure: self.structure,
            style_dot: self.opt_style_dot.unwrap_or("+"),
            padding: self.opt_padding,
        }
    }
}
#[test]
fn local_test() {
    let mut new_template = TemplateFactory::init();
    new_template
        .padding(vec![2, 4])
        .border("+", BorderWeight::Bold);
    new_template.create_movable();
    let zz = new_template.collect();
    zz.structure.iter().for_each(|x| println!("{}",x));
    println!("this is {:?}", zz);
}
