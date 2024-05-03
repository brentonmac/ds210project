pub struct Stats { //struct of stats
    pub data: Vec<usize>,
}

impl Stats { // implements Stats struct into many different methods
    pub fn new(data: Vec<usize>) -> Stats { // created a Stats struct
        Stats {data }
    }

    pub fn descriptive_stats(&self) -> Vec<f64> {
        //outputs the minimum, Q1, median, Q3, and maximum of the data
        let mut sorted_data = self.data.clone(); // cloning the data so the sort below does not affect the original data
        sorted_data.sort();
        let n = sorted_data.len();
        let mut stats = vec![0.0; 5];
        stats[0] = sorted_data[0] as f64; //first stat in the desc stats is the minimum, or the first value in the sorted data
        stats[1] = sorted_data[n / 4] as f64; 
        if n % 2 == 0 { //If there is an odd len, the median is the average of the middle point and the point before it
            stats[2] = (sorted_data[n / 2 - 1] + sorted_data[n / 2]) as f64 / 2.0;
        } else {
            stats[2] = sorted_data[n / 2] as f64;
        }
        if n % 4 == 0 { //If the len is not divisble by 4, Q3 = the average of the 75% percentile point and the point before that
            stats[3] = (sorted_data[3 * n / 4 - 1] + sorted_data[3 * n / 4]) as f64 / 2.0;
        } else {
            stats[3] = sorted_data[3 * n / 4] as f64;
        }
        stats[4] = sorted_data[n - 1] as f64; // The last point is the max
        stats
    }

    pub fn mean(&self) -> f64 {
        //finds the mean of the data by summing the values and dividing by the length
        let sum: usize = self.data.iter().sum();
        let mean = sum as f64 / self.data.len() as f64;
        mean
    }

    pub fn stdev(&self) -> f64 {
        //finds the standard deviation of the data
        let m = self.mean();
        let variance = self.data.iter().map(|&x| (x as f64 - m).powi(2)).sum::<f64>() / self.data.len() as f64;
        variance.sqrt()
    }

    pub fn zscores(&self) -> Vec<f64> {
        //finds the z-score of each data point based on the mean and standard deviation
        if self.data.is_empty() {
            return Vec::new();
        }
        let m = self.mean();
        let sd = self.stdev();
        let mut scores = vec![];
        for i in &self.data {
            scores.push(((*i as f64)-m)/sd); // z-score is data - mean / standard deviation
        }
        scores
    }
}
