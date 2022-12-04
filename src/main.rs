use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
use std::path::Path;

struct SectionBounds{
    upper:i32,
    lower:i32,
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldnt open {}: {}",display,why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why)=> panic!("could read {}: {}",display,why),
        Ok(_)=> print!("contains:\n{}",s ),
    }

    println!();
    let split_sting = s.split("\n");
    let pairs = split_sting.collect::<Vec<&str>>();
    let mut overlaps = 0;
    for s in pairs.iter(){
        let elf_pair = s.split(",");
        let elf_pair = elf_pair.collect::<Vec<&str>>();
        let b1 = get_bounds(elf_pair.index(0));
        let b2 = get_bounds(elf_pair.index(1));
        if compare_bounds(&b1, &b2) || compare_bounds(&b2, &b1){
            overlaps = overlaps +1;
            println!("overlaps {}",overlaps);
        }
    }
    println!("\noverlaps {}",overlaps);
} 
    

    fn compare_bounds(bound1:&SectionBounds,bound2:&SectionBounds)->bool{
        let contains = bound1.lower <= bound2.lower && bound1.upper >= bound2.upper;
        let overlap = bound1.lower <= bound2.upper &&  bound1.upper >= bound2.upper;
        contains ||overlap
    }

    fn get_bounds(input: &str) ->SectionBounds{
        let bds = input.split("-");
        let bds = bds.collect::<Vec<&str>>();
        let mut bound = SectionBounds{lower : 0,upper : 0};

        bound.lower = bds.index(0).parse().unwrap();

        bound.upper = bds.index(1).parse().unwrap();
        

        return bound;
    }


    
