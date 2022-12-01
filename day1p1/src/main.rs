use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;
use std::collections::BinaryHeap;


// idk what this does yet...
#[derive(Copy, Clone, Eq, PartialEq)]
struct Calories {
    count: u32
}

impl Ord for Calories {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count < other.count {
            return Ordering::Greater;
        } else if self.count > other.count {

            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Calories {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let target_file = &args[1];
    let target_path = Path::new(target_file);

    if !Path::exists(target_path) {
        return println!("{target_file} does not exist, please verify the arguments");
    }
    println!("Reading {target_file}...");

    let contents = fs::read_to_string(&target_path).expect("Error reading");
    let mut contents = contents.split('\n');
    
    let mut t_max:  u32 = 0;

    let mut priority_queue = BinaryHeap::new();
    
    loop {
        match contents.next() {
            Some(t) => { 
                
                if t.len() == 0 {
                    priority_queue.push(Calories {count: t_max});
                    t_max = 0;

                    if priority_queue.len() > 3 {
                        priority_queue.pop();
                    }

                    continue;
                }
                let temp: u32 = str::parse(t).expect("Should be a number");
                t_max += temp;
             },
            None => {
                let mut final_calories: u32 = 0;
                while let Some(Calories { count }) = priority_queue.pop() {
                    final_calories += count;
                }
                println!("{final_calories}");
                break;
            },
        }
        
    }


    
}
