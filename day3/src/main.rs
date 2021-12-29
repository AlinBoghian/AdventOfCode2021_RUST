
use std::{i32, i64, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
fn main(){

    let filename = String::from("input.txt");
    let filename2 = filename.clone();

    let rating = part1(filename);
    println!("rating part1 is {}",rating);
    let rating = part2(filename2);
    println!("rating part2 is {}",rating);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn part2(filename:String) -> i64{


    let mut oxygen_rating:i64 = 0;
    let mut scrubber_rating:i64 = 0;
    let mut data:Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename.clone()){
        let bitsize;

        for linerez in lines{
            match linerez {
                Ok(line) => { data.push(line); }
                _ => {}
            };
        }

        bitsize = data[0].len();
        let mut pos = 0;
        let cycl = cyclic_curr(bitsize as u32);
        let mut data_oxygen = data.clone();
        let mut data_scrubber = data;

        while data_oxygen.len() > 1 {
            let rez = data_oxygen.clone().into_iter()
                .fold(0,|a,b| fold_commbit(a,b,pos));
            let bit_oxygen =
                if rez < 0 {
                    0
                }else{
                    1
                };
            data_oxygen.retain(|str|filter_commbit_curry(pos,bit_oxygen)(str));
            cycl(&mut pos);
        }
        if let Ok(value) = i64::from_str_radix(&data_oxygen[0].clone(),2){
            oxygen_rating = value;
        }
        pos = 0;
        while data_scrubber.len() > 1 {
            let rez = data_scrubber.clone().into_iter()
                .fold(0,|a,b| fold_commbit(a,b,pos));
            let bit_scrubber =
                if rez < 0 {
                    1
                }else{
                    0
                };
            data_scrubber.retain(|str|filter_commbit_curry(pos,bit_scrubber)(str));
            cycl(&mut pos);
        }
        if let Ok(value) = i64::from_str_radix(&data_scrubber[0].clone(),2){
            scrubber_rating = value;
        }
    }
    oxygen_rating * scrubber_rating
}
fn fold_commbit(amm:i32, str:String, pos:u32) -> i32 {
    let bit = match str.chars().nth(pos as usize){
        Some(ch) => match ch.to_digit(10){
            Some(nr ) => {nr}
            None => {exit(-1)}
        }
        None => {exit(-1)}
    };
    if bit == 1 {
        amm + 1
    }else {
        amm - 1
    }
}
fn filter_commbit_curry(pos:u32,combit:i32) -> impl FnMut(&String) -> bool {
    move |str: &String| {
        let opt = str.chars().nth(pos as usize);
        match opt {
            Some(ch) => {
                match ch.to_digit(10) {
                    Some(digit) => { digit == combit as u32 }
                    None => { exit(-1) }
                }
            }
            None => { exit(-1); }
        }
    }
}
fn cyclic_curr(cap:u32) -> impl Fn(&mut u32){
    move |nr| cyclic_incr(nr,cap)
}

fn cyclic_incr(nr: &mut u32, cap:u32){
    *nr = (*nr + 1) % cap;
}


fn part1(filename:String) -> i32{
    let mut gamma: Vec<i32> = vec![0, 1];
    let mut bitsize: usize = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut iter = lines.peekable();
        if let Some(Ok(firstr)) = iter.peek() {
            bitsize = firstr.len();
            gamma = vec![0; bitsize];
        } else { exit(-1); }
        for line in iter {
            if let Ok(bitstring) = line {
                for (i, ch) in bitstring.chars().enumerate() {
                    if let Some(bit) = ch.to_digit(10) {
                        if bit == 1 {
                            gamma[i] = gamma[i] + 1;
                        } else if bit == 0 {
                            gamma[i] = gamma[i] - 1;
                        } else {
                            panic!("file contains characters other than 0 & 1");
                        }
                    } else { exit(-1) };
                }
            }
        }
    }
    let mut gammarate: i32 = 0;
    let mut epsilonrate: i32 = 0;
    for i in 0..bitsize {
        if gamma[i] >= 0 {
            gammarate |= 1 << (bitsize - i - 1);
        } else {
            epsilonrate |= 1 << (bitsize - i - 1);
        }
    }
    gammarate * epsilonrate
}