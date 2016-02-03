extern crate rand;
use rand::Rng;
use std::vec;
use std::cmp::Ordering;
use std::io;

//fn printVec(data: &[i32]) {
//    for x in data.iter() {
//        print!("{} ", x);
//    }
//}

fn swap<T>(data: &mut [T], lhs: usize, rhs: usize) 
    where T: Copy
{
    let tmp : T = data[lhs];
    data[lhs] = data[rhs];
    data[rhs] = tmp;
}

fn quick_sort<T>(data: &mut [T], comp: &Fn(&T, &T) -> Ordering)
    where T: Copy
{
    let dataLen = data.len();
    if (dataLen > 1) {
        //println!("");
        
        //print!("{}", "on enter: ");
        //printVec(data);
        //println!("");
        
        let mut currentLow = 0;
        let mut currentHigh = 0;
        let mut currentPos = 0;
        
        {
            let (head, tail) = data.split_at_mut(1);
            let tailLen = tail.len();
            let centerElem = &head[0];
            //println!("{}: {}", "center  : ", centerElem);
            
            currentLow = 0;
            currentHigh = tail.len() - 1;
            currentPos = 0;
            
            if (tailLen > 1) {
                loop {
                    if (currentLow == currentHigh) {
                        break;
                    }
                    match (comp(&tail[currentPos], centerElem)) {
                        Ordering::Greater | Ordering::Equal => {
                            swap(tail, currentPos, currentHigh);
                            currentHigh = currentHigh - 1;
                        },
                        Ordering::Less => {
                            swap(tail, currentPos, currentLow);
                            currentLow = currentLow + 1;
                            currentPos = currentPos + 1;
                        },
                    }
                    
                    //print!("{}", "on oper : ");
                    //printVec(head);
                    //printVec(tail);
                    //println!("");
                }
            }
        }
        
        currentHigh = currentHigh + 1;
        currentLow = currentLow + 1;
        
        match (comp(&data[0], &data[currentLow])) {
            Ordering::Greater => swap(data, 0, currentLow),
            Ordering::Less | Ordering::Equal => swap(data, 0, currentLow - 1),
        }
        
        //println!("hi low : {} {}", currentHigh, currentLow);
        
        //print!("{}", "on exit : ");
        //printVec(data);
        //println!("");
        //println!("");
        
        //println!("from {} to {}", 0, currentHigh);
        quick_sort(&mut data[0..currentHigh], comp);
        //println!("from {} to {}", currentHigh, dataLen);
        quick_sort(&mut data[currentHigh..dataLen], comp);
    }
}

fn comp(l: &i32, r: &i32) -> Ordering { l.cmp(r) }

fn main() {
    let mut rndGen = rand::thread_rng();
    for x in 0..1000 {
        let count = rndGen.gen_range(10, 100);
        let mut buf : Vec<i32> = Vec::with_capacity(count as usize);
        for i in 0..count {
            buf.push(rndGen.gen_range(0, 1000));
        }
        quick_sort(&mut buf, &comp);
        //printVec(&buf);
        //println!("");
        
        for i in 0..(count - 1) {
            if (buf[i] > buf[i + 1]) {
                println!("{}", "Error!");
            }
        }
    }
}
