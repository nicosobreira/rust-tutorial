pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        let min = 1;
        let max = 100;
        if value < min || value > max {
            return Err(format!("Your guess is out of the range {min} and {max}"));
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
