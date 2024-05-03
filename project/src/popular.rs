pub fn popularity_scale(z_scores: Vec<f64>) -> Vec<usize> {
    // returns a vector of popularity scale, which is based on the z-score of node degree
    let mut scales = vec![];
    for &z in &z_scores {
        let scale = match z { // matching each level of z-score with a popularity scale
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
    //returns a vector of the nodes that are considered celebrities, which are people with a 7 on popularity scale
    let mut celebs = vec![];
    for i in 0..pop_scale.len() {
        if pop_scale[i] == 7 { // if the popularity score is 7, push that node to the vector
            celebs.push(i);
        }
    } 
    celebs
}