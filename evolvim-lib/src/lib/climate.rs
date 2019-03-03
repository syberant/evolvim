use std::f64::consts::PI;

#[derive(Serialize, Deserialize)]
pub struct Climate {
    temperature: f64,
    min_temperature: f64,
    max_temperature: f64,
}

impl Climate {
    /// Returns the growth rate (temperature) for the given time.
    pub fn get_growth_rate(&self, time: f64) -> f64 {
        let temp_range = self.max_temperature - self.min_temperature;
        return self.min_temperature + temp_range * 0.5
            - temp_range * 0.5 * ((time % 1.0) * 2.0 * PI).cos();
    }

    pub fn get_growth_over_time_range(&self, time: f64, last_updated: f64) -> f64 {
        let temp_range = self.max_temperature - self.min_temperature;
        let m = self.min_temperature + temp_range * 0.5;

        return (time - last_updated) * m
            + (temp_range / PI / 4.0) * ((PI * 2.0 * last_updated).sin() - (PI * 2.0 * time).sin());
    }

    pub fn update(&mut self, time: f64) {
        self.temperature = self.get_growth_rate(time);
    }

    pub fn new(min: f64, max: f64) -> Self {
        Climate {
            temperature: 0.0,
            min_temperature: min,
            max_temperature: max,
        }
    }
}

// All functions simply returning properties.
impl Climate {
    pub fn get_temperature(&self) -> f64 {
        return self.temperature;
    }
}
