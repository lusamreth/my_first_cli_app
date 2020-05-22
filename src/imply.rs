use std::time::Duration;
pub fn mainrunner(){

}
struct Date(String,i32,i32,i32);
struct User<'a>{
    name:String,
    profile_pic:Path,
    desc:String,
    timestamp:f64,
    born:Date,
    ban_duration:&'a str,
    rank:String,
    status:Userstatus,
    telephone:i32
}

enum Userstatus{
    Online,
    Away,
    Offline,
    Banned
}

struct Path{
    name:String,
    dir:String
}
trait UserUtility{
    fn ban<T>(&mut self,commander:User,duration:String) -> Result<(),String> where T:PartialEq;
}
fn st(x:&str) -> String{
    String::from(x)
}
struct Message{
    commander:String,
    text:String,

}
impl UserUtility for User<'_>{
    fn ban<T>(&mut self,commander:User,duration:String) -> Result<(),String> where T:PartialEq{
        let scope_banned:[&str;4] = ["3 days","1 day","7 days","30 days"];
        
        for x in scope_banned.iter(){
            if x == &duration{
                self.ban_duration = x
            }
        }
        if duration == "permenant".to_string() && commander.rank == "admin"{
            self.ban_duration = "permenant";
            Ok(())
        }else{
            let message = "Only admin has the permission to perma ban!".to_string();
            Err(message)
        }
     
        // self.status = Userstatus::Banned;
        
        // Ok(())
    }
}