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
    let mut rndGen = rand::thread_rng();
    for x in 1..10000 {
        let count = rndGen.gen_range(10, 500);
        let mut data : Vec<i32> = Vec::with_capacity(count as usize);
        for i in 0..count {
            data.push(i as i32);
        }
        let targetIndex : isize = rndGen.gen_range(0, count);
        
        let searchResult = binarySearch(data[targetIndex as usize], &data, 0);
        
        if (searchResult != targetIndex) {
            println!("Error!");
        }
    }
}
