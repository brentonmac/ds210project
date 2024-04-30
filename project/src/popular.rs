pub fn popularity_scale(z_scores: Vec<f64>) -> Vec<usize> {
    let mut scales = vec![0;z_scores.len()];
    for i in 0..z_scores.len() {
        if z_scores[i] < -2.0 {
            scales[i] = 1;
        } else if z_scores[i] < -1.0 {
            scales[i] = 2;
        } else if z_scores[i] < 0.0 {
            scales[i] = 3;
        } else if z_scores[i] < 1.0 {
            scales[i] = 4;
        } else if z_scores[i] < 2.0 {
            scales[i] = 5;
        } else if z_scores[i] < 3.0 {
            scales[i] = 6;
        } else {
            scales[i] = 7;
        }
    }
    scales
}