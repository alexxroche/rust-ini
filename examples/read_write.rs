/*
  cargo init ini_example
  cd ini_example
  echo 'rust-ini = "0.15"' >> Cargo.toml
  <replace src/main.rs with this file>
  for i in {fmt,test,check,build,run}; do cargo $i; done
*/
extern crate ini;
use ini::Ini;

fn write_conf() {
    let mut conf = Ini::new();
    conf.with_section(None::<String>)
        .set("encoding", "utf-8");
    conf.with_section(Some("User"))
        .set("given_name", "Tommy")
        .set("family_name", "Green")
        .set("unicode", "Raspberry树莓");
    conf.with_section(Some("Book"))
        .set("name", "Rust cool");
    conf.write_to_file("conf.ini").unwrap();
 
}

fn read_conf() {
    let conf = Ini::load_from_file("conf.ini").unwrap();

    let section = conf.section(Some("User")).unwrap();
    let tommy = section.get("given_name").unwrap();
    let green = section.get("family_name").unwrap();

    println!("given_name: {:?} family_name: {:?}", tommy, green);

    // iterating
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
        }
    }   
}   

// check if conf.ini exists
use std::fs;
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn main() {
    if ! path_exists("conf.ini") {
        println!("[i] writing conf.ini");
        write_conf();
    }else{
        println!("[w] conf.ini found!");
    }
    read_conf();
}
