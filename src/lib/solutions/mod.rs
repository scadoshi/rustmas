pub mod year_2015;

pub trait Solution: Sized {
    fn new(input: &'static str) -> anyhow::Result<Self>;
    fn part_one(&self) -> Option<String>;
    fn part_two(&self) -> Option<String>;
}
