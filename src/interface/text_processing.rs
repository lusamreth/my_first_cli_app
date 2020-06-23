use std::collections::hash_map::{self, HashMap};
use unicode_segmentation::UnicodeSegmentation;
type Crump = Vec<char>;
#[derive(Debug, Clone)]
pub struct Cluster {
    pub value: Vec<String>,
}
#[derive(Debug)]
pub struct CrumpCluster {
    value: Vec<Crump>,
}
impl Cluster {
    pub fn make_cluster(long_str: &str) -> Cluster {
        let cluster_result = long_str
            .trim()
            .unicode_words()
            .map(|chunck| chunck.to_string())
            .collect::<Vec<String>>();
        return Cluster {
            value: cluster_result,
        };
    }
}

impl CrumpCluster {
    fn chain(&mut self) -> &mut Self {
        return self;
    }
    pub fn break_chunk(word: &str) -> Self {
        let Cc = UnicodeSegmentation::graphemes(word, true)
            .map(|each_str| each_str.chars().into_iter().collect::<Crump>())
            .collect::<Vec<Crump>>();

        return CrumpCluster { value: Cc };
    }
    pub fn insert(&mut self, indx: usize, element: &str) -> &mut Self {
        // let blop = self.value.get_mut(indx).unwrap();
        match self.value.get_mut(indx) {
            Some(val) => {
                element.chars().for_each(|ec| val.push(ec));
            }
            None => eprintln!("Cannot access Non-existed element! at index {}", indx),
        }
        self.chain()
    }

    pub fn delete(&mut self, indx: usize, end: Option<usize>) -> &mut Self {
        let printErr = || eprintln!("Error! : Cannot Delete element at index {}!", indx);

        if let Some(end_indx) = end {
            match self.value.get_mut(indx..end_indx) {
                Some(val) => {
                    val.iter_mut().for_each(|ch| {
                        ch.pop();
                    });
                }
                None => printErr(),
            }
        } else {
            match self.value.get_mut(indx) {
                Some(val) => {
                    val.pop();
                }
                None => printErr(),
            }
        }

        self.chain()
    }
    pub fn get_raw(self) -> Vec<Crump> {
        return self.value;
    }
    pub fn merge_crump(&self) -> String {
        self.value
            .iter()
            .map(|each_crump| each_crump.into_iter().collect::<String>())
            .collect()
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

#[test]
fn cp() {
    let orig = "apple pencile dial apple nasndsm nasmdn a,sdn m,as dna ,sn,adn m,ns,a apple";
    let mut n = CrumpCluster::break_chunk(orig);
    let reg = regex::Regex::new(r"apple").unwrap();
    let arr = reg
        .find_iter(orig)
        .map(|x| (x.start(), x.end()))
        .collect::<Vec<(usize, usize)>>();
    arr.iter().for_each(|x|{
        println!("x {:?}",x);
        n.delete(x.0, Some(x.1));
        n.insert(x.0, "bananana");
    });
    println!("arr  {:?}",arr);
    println!("n {:?}", n.merge_crump());
}
