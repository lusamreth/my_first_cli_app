use std::collections::HashMap;
#[test]
fn txt() {
    let regex = regex::Regex::new(r#".*?"#).unwrap();
    let mut haystack = "apple \"fuckk\" apple bryg apple bruh ".to_string();
    let matched = haystack.match_indices("apple").collect::<Vec<_>>();
    let index_vec = matched.into_iter().map(|x| x.0).collect::<Vec<usize>>();
    index_vec.iter().for_each(|idx|{
        haystack.replace_range(idx..&5, "Bruha");
    });
    println!("NEWHAY {}",haystack);

}

fn replace_haystack(haystack:&mut String,target:&str,replace_with:&str){
    let matched = haystack.match_indices(target).collect::<Vec<_>>();
    let index_vec = matched.into_iter().map(|x| x.0).collect::<Vec<usize>>();
    index_vec.iter().for_each(|idx|{
        haystack.replace_range(idx+1..(idx+target.len()), replace_with);
    });
}
#[test]
fn capture(){
    let re = regex::Regex::new(r"\d++").unwrap();
    let haystack = "1239 931y0 920asdsdw13 92a dsd19 123asda8 1w23";
    for found in re.find_iter(haystack){
        println!("{:?}",(found.start(),found.end()));
    }

}
#[test]
fn test(){
    let mut new_hs = "apple birb tree tree pa tree".to_string();
    replace_haystack(&mut new_hs, "tree", "apple");
    println!("{}",new_hs);
    assert_eq!("apple birb apple apple pa apple",new_hs);
}