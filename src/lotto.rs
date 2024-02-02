use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

// New Type Pattern
pub struct LottoDisplay<'a>(&'a [u32]);

impl<'a> std::fmt::Display for LottoDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🍀")?;
        for (i, num) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " 🍀 ")?;
            }
            write!(f, "{}", num)?;
        }
        write!(f, "🍀")
    }
}

pub struct Lotto<'a> {
    pot: Vec<u32>,
    take: usize,
    rng: &'a mut ThreadRng,
}

impl<'a> Lotto<'a> {
    pub fn new(take: usize, pot_size: u32, rng: &'a mut ThreadRng) -> Self {
        let pot: Vec<u32> = (1..=pot_size).collect();
        Self { pot, take, rng }
    }

    pub fn draw(&mut self) -> LottoDisplay {
        self.pot.shuffle(&mut self.rng);
        LottoDisplay(&self.pot[0..self.take])
    }
}

pub fn call_lotto() {
    let mut rng = thread_rng();
    let mut lotto = Lotto::new(6, 45, &mut rng);
    println!("Lotto numbers: {}", lotto.draw());
    println!("Lotto numbers: {}", lotto.draw());
    println!("Lotto numbers: {}", lotto.draw());
    println!("Lotto numbers: {}", lotto.draw());
    println!("Lotto numbers: {}", lotto.draw());
    println!("Lotto numbers: {}", lotto.draw());

    let mut euro_mil = Lotto::new(5, 50, &mut rng);
    println!("Lotto numbers: {}", euro_mil.draw());
    let mut euro_mil_stars = Lotto::new(2, 12, &mut rng);
    println!("Lotto numbers: {}", euro_mil_stars.draw());
}
