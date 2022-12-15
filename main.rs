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

fn in_poor_elf3()
{
    let mut sum:u32 = 0;

    if let Ok(lines) = read_lines("elf3.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
             
                let mut len = ip.len();
                if len % 2 == 0 
                {
                    len /= 2;
                    
                    let mut a:Vec<u32> = Vec::new();
                    let mut b:Vec<u32> = Vec::new();

                    let mut counter = 0;
                    
                    for c in ip.chars()
                    {                        
                        let mut v = c as u32; 

                        if v >= 97 && v <= 122 
                        {
                            v = v - 96;
                        }
                        else if v >= 65 && v <= 90
                        {
                            v = v - (65 - 27);
                        }
                        
                        if counter < len
                        {
                            a.push(v);
                        }
                        else
                        {
                            b.push(v);
                        }

                        counter += 1;
                    }

                    a.sort();
                    a.dedup();

                    for w in a
                    {
                        let res = b.iter().find(|&&x| x == w);
                        if res != None 
                        {
                            sum += w;                         
                        }                        
                    }

                }
            }
        }
    }

   println!("Sum {}", sum);
}

fn in_poor_elf3_b()
{
    let mut sum:u32 = 0;
   
    let mut a:Vec<u32> = Vec::new();
    let mut b:Vec<u32> = Vec::new();

    let mut elf_counter = 0;

    if let Ok(lines) = read_lines("elf3.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                for ch in ip.chars()
                {                        
                    let mut v = ch as u32; 

                    if v >= 97 && v <= 122 
                    {
                        v = v - 96;
                    }
                    else if v >= 65 && v <= 90
                    {
                        v = v - (65 - 27);
                    }
                    
                    a.push(v);                        
                }

                a.sort();
                a.dedup();

                b.append(&mut a);
                a.clear();                    

                if elf_counter % 3 == 2 
                {
                    b.sort();

                    let mut previous:u32 = 0;
                    let mut count = 0;

                    for moo in &b 
                    {
                        if previous == *moo
                        {
                            count += 1;        
                        }
                        else
                        {
                            count = 0;
                        }

                        if count >= 2
                        {
                            sum += *moo;
                        }

                        previous = *moo;
                    }

                    b.clear();

                }

                elf_counter += 1;
            }
        }
    }

   println!("Sum {}", sum);
}

use std::iter::FromIterator;

fn in_poor_elf4()
{
    let mut sum:u32 = 0;
   
    if let Ok(lines) = read_lines("elf4.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                let res = Vec::from_iter(ip.split(',').map(String::from));

                let left:Vec<&str> = res[0].split('-').collect();

                let a = left[0].parse::<u32>().unwrap();
                let b = left[1].parse::<u32>().unwrap();

                let right:Vec<&str> = res[1].split('-').collect();

                let c = right[0].parse::<u32>().unwrap();
                let d = right[1].parse::<u32>().unwrap();

                /*
                if inside(a,b,c,d)
                {
                    sum += 1;
                }
                else if inside(c,d,a,b)
                {
                    sum += 1;
                }
                */

                if intersect(a,b,c,d) 
                {
                    sum += 1;
                }
            }
        }
    }

   println!("Sum {}", sum);
}

fn inside(a: u32, b:u32, c:u32, d:u32) -> bool
{
    if d > b || c > b
    {
        return false;
    }

    if c < a || d < a
    {
        return false;
    }

    return true;
}

fn intersect(a: u32, b:u32, c:u32, d:u32) -> bool
{
    return a <= d && b >= c;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() 
{
    in_poor_elf4();
}
