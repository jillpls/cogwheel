use nalgebra::dvector;
use cogwheel::*;

fn main() {
    let mut cw = CogWheel::new(vec![2,10,10,10,2], None);
    cw.init_random_weights();
    println!("{:?}", cw);
    let r = cw.run(dvector!(1.0, 2.0)).unwrap_or_else(|x| {
        println!("{:?}", x);
        panic!();
    });
    println!("{}", r);
}