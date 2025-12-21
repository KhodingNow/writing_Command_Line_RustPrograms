use clap::{Arg, ArgAction, Command} 
use clap::Command; //1


fn main() {
   let _matches  = Command::new("echor") // 2
   .version("0.1.0")
   .author("sirmba<sirmba@gmail.com>")
   .about("Rust version of 'echo'")
   .get_matches(); //6
}
