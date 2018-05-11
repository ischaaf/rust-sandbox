
use std::str;
use std::env;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    reverse_lines(fname);
}

fn reverse_lines(fname: &String) {
    let mut f = File::open(fname).expect("file not found");
    let mut buf = [0; 10];
    let mut offset = f.seek(SeekFrom::End(-10)).expect("failed to seek in file");
    let mut read_len: i64 = 0;
    f.read(&mut buf).expect("error reading file");
    while read_len > 0 || offset > 0 {
        {
            let s = str::from_utf8(&buf[0..(read_len as usize)]).expect("invalid unicode sequence");
            let chunk = String::from(s);
            print!("{}", s);
        }
        //println!("read_len:{},offset:{}", read_len, offset);
        read_len = min(10, offset) as i64;
        let seek_dist = read_len + 10;
        offset = f.seek(SeekFrom::Current(0 - seek_dist)).expect("failed to seek in file");
        f.read(&mut buf).expect("error reading file");
    }
}
