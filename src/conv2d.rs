pub fn conv2d_forward(input: &Vec<Vec<f32>>,  kernel: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let h = input.len();
    let w = input[0].len();
    let k = kernel.len();

    let out_h = h - k + 1;
    let out_w = w - k + 1;

    let mut output = vec![vec![0.0; out_w]; out_h];

    for i in 0..out_h {
        for j in 0..out_w {
            let mut sum = 0.0;

            for u in 0..k {
                for v in 0..k {
                    sum += input[i + u][j + v] * kernel[u][v];
                }
            }

            output[i][j] = sum;
        }
    }

    
    output
}
