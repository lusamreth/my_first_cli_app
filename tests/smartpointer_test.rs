use cpx2::interface::gen_whitespace;
use std::collections::HashMap;
#[test]
fn txt() {
    let regex = regex::Regex::new(r#".*?"#).unwrap();
    let mut haystack = "apple \"fuckk\" apple bryg apple bruh ".to_string();
    let matched = haystack.match_indices("apple").collect::<Vec<_>>();
    let index_vec = matched.into_iter().map(|x| x.0).collect::<Vec<usize>>();
    index_vec.iter().for_each(|idx| {
        haystack.replace_range(idx..&5, "Bruha");
    });
    println!("NEWHAY {}", haystack);
}

fn replace_haystack(haystack: &mut String, target: &str, replace_with: &str) {
    let matched = haystack.match_indices(target).collect::<Vec<_>>();
    let index_vec = matched.into_iter().map(|x| x.0).collect::<Vec<usize>>();
    index_vec.iter().for_each(|idx| {
        haystack.replace_range(idx + 1..(idx + target.len()), replace_with);
    });
}
#[test]
fn capture() {
    let re = regex::Regex::new(r"\d++").unwrap();
    let haystack = "1239 931y0 920asdsdw13 92a dsd19 123asda8 1w23";
    for found in re.find_iter(haystack) {
        println!("{:?}", (found.start(), found.end()));
    }
}
#[test]
fn test() {
    let mut new_hs = "|0|apple birb tree tree pa tree".to_string();
    replace_haystack(&mut new_hs, "tree", "apple");
    println!("{}", new_hs);
    // assert_eq!("apple birb apple apple pa apple",new_hs);
    let bordered = incb_text_border(&new_hs);
    display(bordered);
}
fn incb_text_border(txt: &str) -> Vec<String> {
    let lines = txt.lines();
    let width = 160;
    println!("{}",width);

    let mut ovi = Vec::new();
    let mut template = String::new();
    template.push('+');
    for _ in 0..width {
        template.push_str("-")
    }
    template.push('+');
    let bottom = template.clone();
    ovi.push(template);
    txt.lines().into_iter().for_each(|line| {
        let longest = width;
        let repeater = longest - line.len();
        let c = format!("|{}{}|", line, " ".repeat(repeater as usize));
        ovi.push(c.clone());
    });
    ovi.push(bottom);
    println!("{:?}", ovi);
    ovi
    // println!("|{}|",txt);
}
fn incb_text_border_using_vec(txt: Vec<String>) -> Vec<String> {
    let mut max = txt.get(0).unwrap();
    for (_, ct) in txt.iter().enumerate() {
        if ct.len() > max.len() {
            max = ct;
        }
    }
    let width = max.len() as i32;
    let mut ovi = Vec::new();
    let mut template = String::new();
    println!("{}", width);
    template.push('+');
    for _ in 0..width {
        template.push_str("-")
    }
    template.push('+');
    let bottom = template.clone();
    ovi.push(template);
    txt.into_iter().for_each(|line| {
        let longest = width;
        let repeater = longest - (line.len() as i32);
        let c = format!("|{}{}|", line, " ".repeat(repeater as usize));
        ovi.push(c.clone());
    });
    ovi.push(bottom);
    ovi
    // println!("|{}|",txt);
}
#[cfg(test)]
mod practical_test {
    use super::*;
    use cpx2::interface::components;
    use std::fs::File;
    use std::io::Read;
    use std::io::{self, BufReader};
    #[test]
    fn practical() -> io::Result<()> {
        let big_txt = File::open("./big.txt")?;
        let mut result = String::new();
        let mut buffer = BufReader::new(big_txt);
        buffer.read_to_string(&mut result)?;
        let parsed =
            incb_text_border_using_vec(result.lines().into_iter().map(|x| x.trim().to_string()).collect());
        // let parsed = incb_text_border(&result);
        display(parsed);
        Ok(())
    }
}
fn display(tmp: Vec<String>) {
    tmp.iter().for_each(|line| println!("{}", line));
}
