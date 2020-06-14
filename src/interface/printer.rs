use std::io::{self,Write};
use console::{Attribute,Color};

struct TermCfg{
    color:Color,
    background:Color,
    attribute:Attribute
}
type Printer<'a> = Box<dyn FnMut(&str) -> io::Result<()> + 'a>;

impl TermCfg{
    fn new() -> Self{
        TermCfg{
            color:Color::Black,
            background:Color::White,
            attribute:Attribute::Hidden
        }
    }
    fn set_attr(mut self,attr:Attribute) -> Self{
        self.attribute = attr;
        self

    }
    fn generate_buffer(&self) -> io::BufWriter<io::Stdout>{
        let std_out = io::stdout();
        let mut handle = io::BufWriter::new(std_out);
        handle.buffer().windows(4);
        return handle;
    }
    fn gen_println<'a>(self) -> Printer<'a>{
        let mut handle = self.generate_buffer();
        let printer = move |content:&str| -> io::Result<()>{
            console::set_colors_enabled(true);
            let em_style = console::Style::new().bright().attr(self.attribute);
            let colorized = match self.color{
                Color::Red => em_style.red(),
                Color::Blue => em_style.blue(),
                Color::Green => em_style.green(),
                Color::Black | _=> em_style.black()
            };
            let styled_content = colorized.apply_to(content).to_string();
           writeln!(handle,"{}",styled_content)
        };
        return Box::new(printer);
    }
    fn gen_print<'a>(self) -> Printer<'a> {
        let mut handle = self.generate_buffer();
        let printer = move |content:&str| -> io::Result<()>{
           write!(handle,"{}",content)
        };
        return Box::new(printer);
    }
}
