pub enum OutputType {
    Test,
}

impl OutputType {
    pub fn use_output(&self, _value: f64, _env: &mut crate::brain::EnvironmentMut, _time_step: f64) {
        use OutputType::*;

        match self {
            Test => unimplemented!(),
        };
    }
}