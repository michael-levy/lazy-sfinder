use clap::Parser;
use std::cmp::min;
use std::collections::HashSet;
use std::{process};
use std::io::{ErrorKind,Error};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Max number of patterns
    #[arg(short, long, default_value_t = 100)]
    num: i32,
}

/// Represent a perfect clear solution
/// 
/// 'fumen' refers to a link to fumen.zui for the given pattern
/// 
/// It also stores a list of the queues that the solution works for
pub struct Pattern {
    fumen: String,
    qs: HashSet<String>,
}

fn main() {
    let args = Args::parse();
    match read() {
        Ok(result) =>greedy(result, args.num),
        Err(e) => {
            eprintln!("Error reading ../output/path.csv: {}", e);
            process::exit(1);
        }
    }
}

fn read() -> Result<Vec<Pattern>, Error> {
    let file_path = "../output/path.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    let mut data: Vec<Pattern> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let fumen;
        if let Some(parsed_fumen) = record.get(0){
            fumen = parsed_fumen.to_string();
        }else{
            return Err(Error::new(ErrorKind::NotFound, "Incorrect CSV format"));
        }

        let qs:HashSet<String>;
        if let Some(parsed_qs) = record.get(7){
            qs = parsed_qs.split(";")
            .map(|s| s.to_string())
            .collect();
        }else{
            return Err(Error::new(ErrorKind::NotFound, "Queues column not present"));
        }
        data.push(Pattern {
            fumen: fumen,
            qs: qs,
        });
    }

    Ok(data)
}

/// Greedy algorithm to find best set cover
/// 
/// Adds the fumen with the most uncovered queues at each iteration
pub fn greedy(data: Vec<Pattern>, count: i32) {
    let len = data.len() as i32;

    'outer: for i in 1..(min(count, len) + 1) {
        let mut covered_qs: HashSet<String> = HashSet::new();
        let mut covered_fumens: HashSet<String> = HashSet::new();

        // find best partial cover for this number of solutions
        while covered_fumens.len() < i as usize {
            let mut best_cover:i32 = 0;
            let mut best_pattern: Option<&Pattern> = None;
            
            // Get the pattern with the most uncovered queues
            for pattern in data.iter() {
                let cover = pattern.qs.difference(&covered_qs).count() as i32;
                if cover > best_cover {
                    best_cover = cover;
                    best_pattern = Some(pattern);
                }
            }

            if let Some(pattern) = best_pattern {
                covered_qs.extend(pattern.qs.iter().cloned());
                covered_fumens.insert(pattern.fumen.clone());
            }else{
                eprintln!("No more minimals");
                break 'outer;
            }
        }
        println!(
            "Best solution with {} {}",
            i,
            if i > 1 { "solutions:" } else { "solutions:" }
        );
        if covered_qs.len() > 0 {
            println!("{:.3} % cover",( covered_qs.len() as f32 / 5040.0) * 100.0);
            for fumen in covered_fumens {
                println!("{}", fumen);
            }
        }
    }
}