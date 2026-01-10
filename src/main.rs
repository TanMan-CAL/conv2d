mod conv2d;

use conv2d::conv2d_forward;

fn main() {
    let input = vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 2.0],
        vec![1.0, 0.0, 1.0],
    ];
    let kernel = vec![
        vec![1.0, -1.0],
        vec![0.0,  2.0],
    ];
    let output = conv2d_forward(&input, &kernel);


}
