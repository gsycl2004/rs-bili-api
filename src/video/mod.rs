use std::collections::HashMap;
pub mod action;
pub mod info;
pub mod online;


const TABLE: &str = "fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const XOR: i64 = 177451812;
const ADD: i64 = 100618342136696320;
const MAGIC_ARRAY: [usize; 10] = [11, 10, 3, 8, 4, 6, 2, 9, 5, 7];


pub struct Video{
    bvid:String
}


impl Video{
    pub fn from_aid(aid: i64) -> Video{
        let x = (aid ^ XOR) + ADD;
        let mut array = "BV          ".chars().collect::<Vec<_>>();
        for (i, &idx) in MAGIC_ARRAY.iter().enumerate() {
            let n = (x / 58i64.pow(i as u32)) % 58;
            let c = TABLE.chars().nth(n as usize).unwrap();
            array[idx] = c;
        }
        Video{
            bvid:array.iter().collect()
        }
    }

    pub fn from_bvid(bvid: &str) ->Video{
        Video{ bvid:bvid.to_string()}
    }

    pub fn to_aid(&self) -> i64 {
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





