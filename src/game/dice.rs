use rand::Rng;
pub struct Dice();
impl Dice {
    pub fn roll(sides: u32, modifier: i32) -> u32 {
        let mut rng = rand::thread_rng();
        let roll: i32 = rng.gen_range(1..sides + 1) as i32;
        let val = roll + modifier;

        if val < 1 {
            1
        } else {
            val as u32
        }
    }

    /// Rolls 4d6 and drops the lowest value, returning the sum of the remaining 3.
    pub fn stat_roll() -> u32 {
        let mut rolls: Vec<u32> = Vec::new();
        for _ in 0..4 {
            rolls.push(Dice::roll(6, 0));
        }

        rolls.sort();
        rolls.remove(0);
        rolls.iter().sum()
    }

    // Rolls dice from a string, e.g. "2d6+1"
    pub fn from_string(s: &str) -> Result<u32, ()> {
        let count = s.split('d').collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .map_err(|_| ())?;
        let sides = s.split('d').collect::<Vec<&str>>()[1]
            .split('+')
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .map_err(|_| ())?;
        let modifier = s.split('d').collect::<Vec<&str>>()[1]
            .split('+')
            .collect::<Vec<&str>>()
            .get(1)
            .or(Some(&"0"))
            .unwrap()
            .parse::<i32>()
            .map_err(|_| ())?;

        let mut total: i32 = 0;
        for _ in 0..count {
            total += Dice::roll(sides, 0) as i32;
        }

        total += modifier;
        if total < 1 {
            Ok(1)
        } else {
            Ok(total as u32)
        }
    }
}
