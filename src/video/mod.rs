use std::collections::HashMap;

mod action;
mod info;

const TABLE: &str = "fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const XOR: i64 = 177451812;
const ADD: i64 = 100618342136696320;
const MAGIC_ARRAY: [usize; 10] = [11, 10, 3, 8, 4, 6, 2, 9, 5, 7];


pub struct Video{
    bvid:String
}


impl Video{

    fn to_aid(&self) -> i64 {
        let x =  self.bvid.as_str();
        let mut map = HashMap::new();
        for (i, c) in TABLE.chars().enumerate() {
            map.insert(c, i);
        }
        let mut num = 0i64;
        for (i, &idx) in MAGIC_ARRAY.iter().enumerate() {
            let c = x.chars().nth(idx).unwrap();
            let n = *map.get(&c).unwrap() as i64;
            num += n * 58i64.pow(i as u32);
        }
        (num - ADD) ^ XOR
    }

}





