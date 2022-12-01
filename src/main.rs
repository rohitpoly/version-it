#[macro_use]
extern crate clap;
use clap::App;
use std::fs::read_to_string;
use serde_json;
use std::env::current_dir;
use std::any::type_name;
use std::char::from_digit;


fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main(){

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let release_type = remove_whitespace(matches.value_of("release_type").unwrap());
    println!("{}", release_type);
    print_type_of(&release_type);
    

    let cwd = current_dir().unwrap();
    let path: String = String::from(cwd.to_string_lossy());
    let full_path = path + "/src/package.json";
    // let file = File::open(full_path).expect("file should open read only");
    let package_json = read_to_string(full_path).expect("Unable to read file");
    println!("{}", package_json);

    let mut json: serde_json::Value = serde_json::from_str(&package_json)
        .expect("file should be proper JSON");
    println!("{}", json);
    println!("{}", json["version"]);
    print_type_of(&json["version"].to_string());
    print_type_of(&json["version"]);
    let mut version = json["version"].to_string();

    let char_vec: Vec<char> = version.chars().collect();
    println!("{}", version);
    print_type_of(&version);
    if release_type == "patch" {
        version = release_version(char_vec, 5);
    } else if release_type == "minor" {
        version = release_version(char_vec, 3);
    } else if release_type == "major" {
        version = release_version(char_vec, 1);
    } else {
        print!("Please enter valid release type. [ patch | minor | major ]");
    }

    version.pop();
    if version.len() > 0 {
        version.remove(0);
    }

    println!("{}", version);
    print_type_of(&version);
    json["version"] = serde_json::Value::String(version);
    println!("{}", json);
}
fn release_version(mut char_vec: Vec<char>,index: usize) -> String {

    let increment = char_vec[index].to_digit(10).unwrap() + 1;
    char_vec[index] = from_digit(increment, 10).unwrap();
    let mut index_list = vec![1, 3, 5];
    let remove_index = index_list.iter().position(|x| *x == index).unwrap();
    for i in 0..=remove_index {
        index_list.remove(0);
    }
    for i in index_list.iter() {
        char_vec[*i] = '0';
      }
    let version: String = char_vec.into_iter().collect();
    println!("{}", version);
    return version as String;
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

