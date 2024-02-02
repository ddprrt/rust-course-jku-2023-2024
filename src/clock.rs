// This is a simple clock implementation that can add minutes to it.
#[derive(Debug)]
struct Clock {
    hours: u32,
    minutes: u32,
}

impl Clock {
    pub fn new(hours: u32, minutes: u32) -> Self {
        let mut clock = Self { hours, minutes };
        clock.normalize();
        clock
    }

    fn normalize(&mut self) {
        let mut hours = self.hours;
        let mut minutes = self.minutes;

        if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }

        if hours >= 24 {
            hours %= 24;
        }

        self.hours = hours;
        self.minutes = minutes;
    }

    pub fn add_minutes(&mut self, min: u32) {
        self.minutes += min;
        self.normalize();
    }

    pub fn get_minutes(&self) -> u32 {
        self.minutes + self.hours * 60
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl std::ops::Add for Clock {
    // Associated Type
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}

impl std::ops::Add<u32> for Clock {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Clock::new(self.hours, self.minutes + rhs)
    }
}

impl std::ops::Add<Clock> for u32 {
    type Output = Clock;

    fn add(self, rhs: Clock) -> Self::Output {
        Clock::new(rhs.hours, rhs.minutes + self)
    }
}

impl std::ops::AddAssign for Clock {
    fn add_assign(&mut self, rhs: Self) {
        self.add_minutes(rhs.get_minutes());
        self.normalize();
    }
}

impl std::ops::AddAssign<u32> for Clock {
    fn add_assign(&mut self, rhs: u32) {
        self.add_minutes(rhs);
        self.normalize();
    }
}

pub fn call_clock_example() {
    let _my_clock = Clock {
        hours: 10,
        minutes: 43,
    };
    let mut my_clock = Clock::new(23, 63);
    //println!("{my_clock}");
    my_clock.add_minutes(57);
    println!(" {my_clock}");

    let clock = Clock::new(10, 20);
    println!("+{clock}");

    my_clock += clock;
    println!("------");
    println!(" {my_clock}");
    println!("+ 10 min");
    let my_clock = 10 + my_clock;
    println!("------");
    println!(" {my_clock}");
}
