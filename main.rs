use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn in_poor_elf1()
{
    let mut vec:Vec<i32> = Vec::new();
    
    let mut running:i32 = 0;

    if let Ok(lines) = read_lines("data.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                if ip.len()<2
                {
                    vec.push(running);
                    running = 0;                 
                }
                else
                {
                    let value = ip.parse::<i32>().unwrap();
                    running += value;
                }
            }
        }
    }

    let mut total:i32 = 0;

    vec.sort_by(|a, b| b.cmp(a));

    for n in 0usize..3
    {
        println!("{}", vec[n]);
        total += vec[n];
    }

    println!("Total {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() 
{
    in_poor_elf1();
}
