extern crate neuroflow;

use neuroflow::data::DataSet;
use neuroflow::FeedForward;

fn main() {

    let mut l = FeedForward::new([2, 10, 5, 1]);

    let mut ds = DataSet::new();
    ds.push(&[5.0, 10.0], &[2.0]);
    ds.push(&[2.5, 5.0], &[2.0]);
    ds.push(&[15.0, 30.0], &[2.0]);
    ds.push(&[13.0, 26.0], &[2.0]);
    ds.push(&[2.0, 8.0], &[4.0]);
    ds.push(&[2.5, 10.0], &[4.0]);
    ds.push(&[2.0, 30.0], &[15.0]);
    ds.push(&[10.0, 200.0], &[20.0]);

    l.train(&ds, 100);

    println!("{:?}", l.calc(&[2.0, 8.0]));

}