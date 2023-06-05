use rand::Rng;

pub struct ChinkoGenerator {
    is_chinko: [u8; 3],
    chinko_dict: [&'static str; 3],
    chinko_list: Vec<u8>,
}

impl ChinkoGenerator {
    pub fn new() -> ChinkoGenerator {
        ChinkoGenerator {
            is_chinko: [0, 1, 2],
            chinko_dict: ["ち", "ん", "こ"],
            chinko_list: Vec::new(),
        }
    }

    pub fn gen_triplet(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let triplet: Vec<u8> = (0..3).map(|_| rng.gen_range(0..=2)).collect();
        self.chinko_list.extend_from_slice(&triplet);
        triplet == self.is_chinko
    }
}

impl std::fmt::Display for ChinkoGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let word: String = self
            .chinko_list
            .iter()
            .map(|elm| self.chinko_dict[*elm as usize].to_string())
            .collect();
        write!(f, "{}", word)
    }
}

// fn main() {
//     let mut generator = ChinkoGenerator::new();
//     generator.gen_triplet();
//     println!("{}", generator);
// }
