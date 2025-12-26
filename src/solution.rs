pub trait Solution {
    const NAME: &'static str;

    type OutputP1 = usize;
    type OutputP2 = usize;

    fn part_one(&self, inp: &str) -> Self::OutputP1;

    fn part_two(&self, inp: &str) -> Self::OutputP2;

    fn run(&self, inp: String);

    #[inline]
    #[must_use]
    fn name(&self) -> &'static str {
        Self::NAME
    }
}
