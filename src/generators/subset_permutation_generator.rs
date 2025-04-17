use super::{
    permutation_generator::PermutationGenerator,
    subset_generator::{KVPairIterator, KeyCount},
};
use crate::base_types::numbers::CountdownNumberBaseType;
pub struct SubsetPermutationGenerator<T: CountdownNumberBaseType> {
    subsets: Vec<Vec<(T, usize)>>,
    permutation_generator: Option<PermutationGenerator<T>>,
}
impl<T: CountdownNumberBaseType> SubsetPermutationGenerator<T> {
    pub fn new<E: IntoIterator<Item = T>>(t: E) -> Self {
        let mut subsets = KVPairIterator::from(KeyCount::from_iter(t)).collect::<Vec<_>>();
        subsets.sort_by_cached_key(|a| -(a.iter().map(|(_, a)| *a).sum::<usize>() as isize));

        Self {
            subsets,
            permutation_generator: None,
        }
    }
}
impl<T: CountdownNumberBaseType> Iterator for SubsetPermutationGenerator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.permutation_generator.is_none() {
            if let Some(b) = self.subsets.pop() {
                self.permutation_generator = Some(PermutationGenerator::from_iter(b));
            } else {
                return None;
            }
        }
        if let Some(a) = self.permutation_generator.as_mut().unwrap().next() {
            return Some(a);
        } else {
            self.permutation_generator = None
        }

        self.next()
    }
}

#[test]
fn test() {
    let a: Vec<usize> = vec![1, 2, 3, 1, 2, 3];
    for i in SubsetPermutationGenerator::new(a) {
        println!("{i:?}");
    }
}
