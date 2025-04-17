use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::HashSet;

use crate::base_types::numbers::CountdownNumberBaseType;

#[derive(Debug, Clone)]

pub struct UniquePermutationGenerator<T: Sized + Clone> {
    a: Vec<T>,
    c: Vec<usize>,
    i: usize,
}

impl<T: Sized + Clone> UniquePermutationGenerator<T> {
    pub fn new<E: IntoIterator<Item = T>>(a: E) -> Self {
        let a: Vec<T> = a.into_iter().collect();

        Self {
            c: vec![0; a.len()],
            a,
            i: 0,
        }
    }
}

impl<T: Sized + Clone> Iterator for UniquePermutationGenerator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.i == 0 {
            self.i = 1;

            return Some(self.a.clone());
        }

        let i = &mut self.i;

        while *i < self.a.len() {
            if self.c[*i] < *i {
                if *i % 2 == 0 {
                    self.a.swap(0, *i);
                } else {
                    self.a.swap(self.c[*i], *i);
                }

                self.c[*i] += 1;

                *i = 1;

                return Some(self.a.clone());
            } else {
                self.c[*i] = 0;

                *i += 1;
            }
        }

        None
    }
}

#[test]

fn test_unique_generator() {
    let item_count: u64 = 100;

    let value = UniquePermutationGenerator::new(1..item_count);

    let mut permutation_count: u64 = 0;

    for _ in value {
        permutation_count += 1;
    }

    assert_eq!(permutation_count, (1..item_count).product())
}

#[derive(Debug)]

pub struct PermutationGenerator<T: CountdownNumberBaseType> {
    groups: BTreeMap<usize, Option<usize>>,
    elements: BTreeMap<usize, T>,
    unique: UniquePermutationGenerator<usize>,
}

impl<T: CountdownNumberBaseType> FromIterator<(T, usize)>
    for PermutationGenerator<T>
{
    fn from_iter<E: IntoIterator<Item = (T, usize)>>(iter: E) -> Self {
        let mut groups = BTreeMap::new();

        let mut elements = BTreeMap::new();

        let mut i = 0;

        for (t, count) in iter {
            let mut group = None;

            for _ in 0..count {
                groups.insert(i, group);

                elements.insert(i, t);

                group = Some(i);

                i += 1;
            }
        }

        Self {
            groups,
            elements,
            unique: UniquePermutationGenerator::new(0..i),
        }
    }
}

impl<T: CountdownNumberBaseType> Iterator for PermutationGenerator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        'a: loop {
            let a = self.unique.next()?;

            let value_index = BTreeMap::from_iter(
                a.clone().into_iter().enumerate().map(|(a, b)| (b, a)),
            );

            for (current_value, current_position) in value_index.iter() {
                if let Some(Some(other_value)) = self.groups.get(current_value)
                {
                    let other_index = value_index.get(other_value).unwrap();

                    if other_index > current_position {
                        continue 'a;
                    }
                }
            }

            let a: Vec<T> = a.into_iter().map(|a| self.elements[&a]).collect();

            return Some(a);
        }
    }
}

#[test]

fn test() {
    let x = [(1u8, 2usize), (2, 1), (3, 1), (4, 1), (5, 3)];

    let perm_count = x.iter().map(|(_, a)| *a);

    let perms: usize = (1..=perm_count.clone().sum()).product();

    let perms_below: usize =
        perm_count.map(|a| (1..=a).product::<usize>()).product();

    let target = perms / perms_below;

    let mut seta = HashSet::new();

    let mut actual = 0;

    for item in PermutationGenerator::from_iter(x) {
        eprintln!("{:02?}", item);

        if !seta.insert(item) {
            panic!();
        }

        actual += 1;
    }

    assert_eq!(target, actual);
}
