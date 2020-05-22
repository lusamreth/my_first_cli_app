extern crate cpx2;
#[test]
fn it_adds_two() {
    assert_eq!(4, cpx2::add_two(2));
}


pub trait TextPos {
    fn write(&self);
    fn finder(&self,content:&str,replace:&str)  -> String;
}
// self is target,content is a haystack!
impl TextPos for &str{
    fn write(&self){
        println!("This is string {}",self);
    }
    fn finder(&self,content:&str,replace:&str) -> String{
        // for line in content.lines(){
        //     let horizontal = line.split_whitespace().collect::<Vec<&str>>();
            
        // }
        content.replace(self, replace)
    }
}
fn hay<T>(b:T,ct:&str) where T:TextPos + std::fmt::Display{
    let aye = b.finder(ct,"fk");
    println!("yooooo {}",aye);
}

#[test]
fn test_textPos(){
    let new_stuff = hay::<&str>("hey","hsjheydasddhasdjasd hey hey hey");
    println!("P result : {:?}",new_stuff);
}