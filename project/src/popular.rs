pub fn popularity_scale(z_scores: Vec<f64>) -> Vec<usize> {
    let mut scales = vec![];
    for &z in &z_scores {
        let scale = match z {
            x if x < -2.0 => 1,
            x if x < -1.0 => 2,
            x if x < 0.0 => 3,
            x if x < 1.0 => 4,
            x if x < 2.0 => 5,
            x if x < 3.0 => 6,
            _ => 7,
        };
        scales.push(scale);
    }
    scales
}

pub fn celeb(pop_scale: Vec<usize>) -> Vec<usize> {
    let mut celebs = vec![];
    for i in 0..pop_scale.len() {
        if pop_scale[i] == 7 {
            celebs.push(i);
        }
    } 
    celebs
}