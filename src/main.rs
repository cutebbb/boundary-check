#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {

}

const BENCH_LEN:usize = 100000;

fn mock_array() -> [i32;BENCH_LEN]{
    let a = [1;BENCH_LEN];
    a
}

fn mock_vec() -> Vec<i32>{
    let vec = vec![1;BENCH_LEN];
    vec
}

#[bench]
fn array_unchecked_sum(b : &mut Bencher){
    let a = mock_array();
    let mut sum = 0;
    b.iter(|| {
        for i in 0..a.len(){
            sum += unsafe { a.get_unchecked(i) };
        }
    });
    println!("{}",sum);
}

#[bench]
fn array_checked_sum(b : &mut Bencher){
    let a = mock_array();
    let mut sum = 0;
    b.iter(|| {
        for i in 0..a.len(){
            sum += *a.get(i).unwrap();
        }
    });
    println!("{}",sum);
}

#[bench]
fn array_checked_slice_add(b : &mut Bencher){
    let a = mock_array();
    let mut sum = 0;
    b.iter(|| {
        for i in 0..a.len(){
            sum += a[i];
        }
    });
    println!("{}",sum);
}

#[bench]
fn vec_unchecked_sum(b: &mut Bencher){
    let vec = mock_vec();
    let mut sum = 0;
    b.iter(|| {
        for i in 0..vec.len(){
            sum += unsafe { vec.get_unchecked(i) };
        }
    });
    println!("{}",sum);
}

#[bench]
fn vec_checked_sum(b: &mut Bencher){
    let vec = mock_vec();
    let mut sum = 0;
    b.iter(|| {
        for i in 0..vec.len(){
            sum += *vec.get(i).unwrap();
        }
    });
    println!("{}",sum);
}

#[bench]
fn vec_checked_slice_add(b: &mut Bencher){
    let vec = mock_vec();
    let mut sum = 0;
    b.iter(|| {
        for i in 0..vec.len(){
            sum += vec[i];
        }
    });
    println!("{}",sum);
}