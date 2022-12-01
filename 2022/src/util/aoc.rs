use std::error::Error;
use std::time::Instant;

pub(crate) trait AoCDay<I, O: std::fmt::Display> {
    fn parse_input(&self, id: &str) -> Result<I, Box<dyn Error>>;
    fn part1(&self, input: &I) -> O;
    fn part2(&self, input: &I) -> O;

    fn run(&self, id: &str) {
        let before_parsing = Instant::now();
        let input = self.parse_input(id).unwrap();

        println!("parsing took {:?}", before_parsing.elapsed());

        let before_part1 = Instant::now();
        let part1 = self.part1(&input);
        println!(
            "day{} part1 (took {:?}): {}",
            id,
            before_part1.elapsed(),
            part1,
        );

        let pefore_part2 = Instant::now();
        let part2 = self.part2(&input);
        println!(
            "day{} part2 (took {:?}): {}",
            id,
            pefore_part2.elapsed(),
            part2,
        );
    }

    fn parse_and_solve_part1(&self, id: &str) -> Result<O, Box<dyn Error>> {
        let input = self.parse_input(id)?;
        Ok(self.part1(&input))
    }

    fn parse_and_solve_part2(&self, id: &str) -> Result<O, Box<dyn Error>> {
        let input = self.parse_input(id)?;
        Ok(self.part2(&input))
    }
}
