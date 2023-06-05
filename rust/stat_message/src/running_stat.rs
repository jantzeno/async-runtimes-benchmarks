// Adapated from https://www.johndcook.com/blog/standard_deviation/
//               https://www.johndcook.com/blog/skewness_kurtosis/

#[derive(Clone, Copy)]
pub struct RunningStat {
    count: u32,
    old_mean: f64,
    new_mean: f64,
    old_sum: f64,
    new_sum: f64,
}

impl RunningStat {
    pub fn new() -> Self {
        Self {
            count: 0,
            old_mean: 0.0,
            new_mean: 0.0,
            old_sum: 0.0,
            new_sum: 0.0,
        }
    }

    pub fn clear(&mut self) {
        self.count = 0;
        self.old_mean = 0.0;
        self.new_mean = 0.0;
        self.old_sum = 0.0;
        self.new_sum = 0.0;
    }

    pub fn data_value_count(&self) -> u32 {
        self.count
    }

    pub fn mean(&self) -> f64 {
        match self.count > 0 {
            true => self.new_mean,
            false => 0.0,
        }
    }

    pub fn variance(&self) -> f64 {
        match self.count > 1 {
            true => self.new_mean / (self.count - 1) as f64,
            false => 0.0,
        }
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn push(&mut self, value: f64) {
        self.count += 1;

        if self.count == 1 {
            self.old_mean = value;
            self.new_mean = value;
            self.old_sum = 0.0;
        } else {
            self.new_mean = self.old_mean + (value - self.old_mean) / self.count as f64;
            self.new_sum = self.old_sum + (value - self.old_mean) * (value - self.new_mean);

            // set up for next iteration
            self.old_mean = self.new_mean;
            self.old_sum = self.new_sum;
        }
    }
}
