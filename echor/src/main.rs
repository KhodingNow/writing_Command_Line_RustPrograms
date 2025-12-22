use clap::{Arg, ArgAction, Command}; // new 1
//use clap::Command; //1


fn main() {
   let matches  = Command::new("echor") // 2
   .version("0.1.0")
   .author("sirmba<sirmba@gmail.com>")
   .about("Rust version of 'echo'")
//   .get_matches(); //6
   .arg(
        Arg::new("text") // new 2
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .num_args(1..),
   )
   .arg(
        Arg::new("omit_newline") // new 3
        .short('n') // 4
        .action(ArgAction::SetTrue) // 5
        .help("Do not print newline"),
        
   )
   .get_matches();

 println!("{:#?}", matches); // 6
}


/*

New

1 = Import Arg, ArgAction, and Command from the clap crate2 = Create a new Arg with the name 'text'. This is a required postional argument that must appeear atleast once and can be repeated.
3 = Create a new Arg with the name 'omit_newline. This is a flag that has only the short short name '-n'.
4 = Note that the single character short vlaue is enclosed in single quotes to denote the 'char' type.
5 = Set the argument to true when the flag is present and false when not.
6= Pretty-print tha arguments.




*/
