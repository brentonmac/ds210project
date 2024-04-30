pub struct Stats {
    pub data: Vec<usize>,
}

impl Stats {
    pub fn new(data: Vec<usize>) -> Stats {
        Stats {data }
    }

    pub fn descriptive_stats(&self) -> Vec<f64> {
        let mut sorted_data = self.data.clone();
        sorted_data.sort();
    
        let n = sorted_data.len();
        let mut stats = vec![0.0; 5];
        stats[0] = sorted_data[0] as f64;
        stats[1] = sorted_data[n / 4] as f64;
        if n % 2 == 0 {
            stats[2] = (sorted_data[n / 2 - 1] + sorted_data[n / 2]) as f64 / 2.0;
        } else {
            stats[2] = sorted_data[n / 2] as f64;
        }
        if n % 4 == 0 {
            stats[3] = (sorted_data[3 * n / 4 - 1] + sorted_data[3 * n / 4]) as f64 / 2.0;
        } else {
            stats[3] = sorted_data[3 * n / 4] as f64;
        }
        stats[4] = sorted_data[n - 1] as f64;
        stats
    }

    pub fn mean(&self) -> f64 {
        let sum: usize = self.data.iter().sum();
        let mean = sum as f64 / self.data.len() as f64;
        mean
    }

    pub fn stdev(&self) -> f64 {
        let m = self.mean();
        let variance = self.data.iter().map(|&x| (x as f64 - m).powi(2)).sum::<f64>() / self.data.len() as f64;
        variance.sqrt()
    }

    pub fn zscores(&self) -> Vec<f64> {
        if self.data.is_empty() {
            return Vec::new();
        }
        let m = self.mean();
        let sd = self.stdev();
        let mut scores = vec![];
        for i in &self.data {
            scores.push(((*i as f64)-m)/sd);
        }
        scores
    }
    
}
