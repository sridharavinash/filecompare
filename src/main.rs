use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::env;

fn main(){
    let args :Vec<String> = env::args().collect();
    let h_vec = load_file(&args[1]);
    let b_vec = load_file(&args[2]);
    let mut found = 0;
    let mut notfound = 0;
    for item in b_vec{
        match h_vec.binary_search(&item){
            Ok(_) => {
                found+=1;
            },
            _ => {
                println!("Not found id:{}",item);
                notfound+=1;
            }
        }
    }
    println!("Found:{}, Missing:{}", found, notfound);
}

fn load_file(filename: &str) -> Vec<i64>{
    let mut v1 = Vec::new();
    if let Ok(lines) = read_lines(filename){
        for l in lines{
            if let Ok(id) = l{
                match id.parse::<i64>(){
                    Ok(x) => v1.push(x),
                    _ => ()
                };
            }
        }
    }
    println!("read {}, got len:{}",filename, v1.len());
    v1.sort();
    return v1;
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}