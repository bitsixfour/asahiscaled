// later
use std::collections::VecDeque;


pub struct Graph {
    pub time: i32,
    pub pressure: i32,
}

pub fn sort_graph(mut vec: Vec<(f64, f64)>, pres: i32) -> Vec<(f64,f64)> {
    let len: i32 = vec.len() as i32;
    let turple: (f64, f64) = (len as f64, pres.into());
    while len > 10 {
        vec.remove(0);
    }
    vec.push(turple);
    vec

}
