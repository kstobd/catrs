use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
// use std::str::replace;


use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(required = false, min_values = 1)]
    files: Vec<String>,

    ///Number all output lines, starting with 1. This option is ignored if -b is in effect.
    #[structopt(short, long)]
    numder: bool,

    ///Number all nonempty output lines, starting with 1.
    #[structopt(short = "b", long)]
    number_nonblank: bool, 

    ///Display a ‘$’ after the end of each line.
    #[structopt(short = "e", long)]
    show_ends: bool,
    
    ///Suppress repeated adjacent blank lines; output just one empty line instead of several.
    #[structopt(short, long)]
    squeeze_blank: bool, 

    ///Display TAB characters as ‘^I’.
    #[structopt(short = "t", long)]
    show_tabs: bool, 

}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    if (opt.numder == false) & (opt.number_nonblank == false) & (opt.show_ends == false) & 
        (opt.squeeze_blank == false) & (opt.show_tabs == false){
        run(&opt)
    }
    if opt.numder == true  & (opt.number_nonblank == false){
        number(&opt)
    }
    if opt.number_nonblank == true | ((opt.numder == true) & (opt.number_nonblank == true)){
        nonempty(&opt)
    }
    if opt.show_ends == true{
        dollar_end(&opt)
    }

    if opt.squeeze_blank == true{
        squeeze_blank(&opt)
    }
    if opt.show_tabs == true{
        show_tabs(&opt)
    }

}

fn run(config: &Opt) {
    for file_name in &config.files{
        let contents = fs::read_to_string(file_name)
            .expect("Should have been able to read the file");
            println!("{}", contents);
    }
}

fn number(config: &Opt){
    let mut num_line = 1;
    for file_name in &config.files{
        let contents = read_lines(file_name)
            .expect("Should have been able to read the file");
        for lines in contents{
            if let Ok(line) = lines {
                println!("{0} {1}", num_line, line);
                num_line += 1;
            }
        }
    }
}

fn nonempty(config: &Opt){
    let mut num_line = 1;
    for file_name in &config.files{
        let contents = read_lines(file_name)
            .expect("Should have been able to read the file");
        for lines in contents{
            if let Ok(line) = lines {
                if line != ""{
                    println!("{0} {1}", num_line, line);
                    num_line += 1;
                } else {
                    println!()
                }
            }
        }
    }
}

fn dollar_end(config: &Opt){
    for file_name in &config.files{
        let contents = read_lines(file_name)
            .expect("Should have been able to read the file");
        for lines in contents{
            if let Ok(line) = lines {
                    println!("{}$", line);
            }
        }
    }
}

fn squeeze_blank(config: &Opt){
    for file_name in &config.files{
        let contents = read_lines(file_name)
            .expect("Should have been able to read the file");
        let mut prew_line:String = "".to_string();
        for lines in contents{
            if let Ok(line) = lines {
                if (line != "") | (prew_line != ""){
                    println!("{}", line);
                }
                prew_line = line;
            }
        }
    }
}

fn show_tabs(config: &Opt){
    for file_name in &config.files{
        let contents = read_lines(file_name)
            .expect("Should have been able to read the file");
        for lines in contents{
            if let Ok(line) = lines {
                    println!("{}", line.replace("\t", "^I"));
            }
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
