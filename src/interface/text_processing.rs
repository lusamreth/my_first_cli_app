use std::collections::hash_map::{self, HashMap};
use unicode_segmentation::UnicodeSegmentation;
type Crump = Vec<char>;
#[derive(Debug, Clone)]
pub struct Cluster {
    value: Vec<String>,
}
#[derive(Debug)]
pub struct CrumpCluster {
    Value: Vec<Crump>,
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

        return CrumpCluster { Value: Cc };
    }
    pub fn insert(&mut self, indx: usize, element: &str) -> &mut Self {
        let blop = self.Value.get_mut(indx).unwrap();
        match self.Value.get_mut(indx) {
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
            match self.Value.get_mut(indx..end.unwrap()) {
                Some(val) => {
                    val.iter_mut().for_each(|ch| {
                        ch.pop();
                    });
                }
                None => printErr(),
            }
        } else {
            match self.Value.get_mut(indx) {
                Some(val) => {
                    val.pop();
                }
                None => printErr(),
            }
        }

        self.chain()
    }
    pub fn get_raw(self) -> Vec<Crump>{
        return self.Value;
    }
    pub fn merge_crump(&self) -> String {
        self.Value
            .iter()
            .map(|each_crump| each_crump.into_iter().collect::<String>())
            .collect()
    }
}

#[test]
fn cp(){
    let n = CrumpCluster::break_chunk("apple pencile dial").get_raw();
    println!("n {:?}",n)
}