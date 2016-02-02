extern crate rand;
use std::cmp::Ordering;
use rand::Rng;
use std::vec;

fn binarySearch(dest : i32, data : &[i32], base : usize) -> isize {
    let dataLen : usize = data.len() as usize;
    //println!("");
    //println!("base: {}", base);
    //println!("datalen: {}", dataLen);
    
    if (dataLen == 0)
    {
        -1
    }
    else
    {
        let middle : usize = dataLen / 2;
        //println!("middle: {}", middle);
        match dest.cmp(&data[middle]) {
            Ordering::Less => {
                //println!("next: {} {}", base, base+middle);
                binarySearch(dest, &data[0..middle], base)
            },
            Ordering::Equal => {
                (middle + base) as isize
            },
            Ordering::Greater => {
                //println!("next: {} {}", base+middle+1, dataLen);
                binarySearch(dest, &data[middle+1..dataLen], base + middle + 1)
            },
        }
    }
}

fn main() {
    for x in 1..10000 {
        let count = rand::thread_rng().gen_range(10, 100);
        let mut data : Vec<i32> = Vec::new();
        for i in 1..count {
            data.push(i as i32);
        }
        let targetIndex : isize = rand::thread_rng().gen_range(0, count - 1);
        
        let searchResult = binarySearch(data[targetIndex as usize], &data, 0);
        
        if (searchResult != targetIndex) {
            println!("Error!");
        }
    }
}
