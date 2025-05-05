use std::collections::BTreeMap;

use crate::base_types::numbers::NumberType;

pub struct KeyCount<T: NumberType> {
    keys: BTreeMap<T, usize>,
}
impl<T: NumberType> IntoIterator for KeyCount<T> {
    type Item = (T, usize);

    type IntoIter = <BTreeMap<T, usize> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}
impl<T: NumberType> FromIterator<T> for KeyCount<T> {
    fn from_iter<E: IntoIterator<Item = T>>(iter: E) -> Self {
        let mut keys = BTreeMap::new();
        for item in iter {
            if let Some(a) = keys.get_mut(&item) {
                *a += 1;
            } else {
                keys.insert(item, 1);
            }
        }
        KeyCount { keys }
    }
}

pub struct KVPairIterator<T: NumberType> {
    keys: BTreeMap<T, (usize, usize)>,
}

impl<T: NumberType> Iterator for KVPairIterator<T> {
    type Item = Vec<(T, usize)>;
    fn next(&mut self) -> Option<Self::Item> {
        let last_non_full = 'a: {
            for (t, (count, max)) in self.keys.iter().rev() {
                if count < max {
                    break 'a *t;
                }
            }
            return None;
        };

        self.keys
            .iter_mut()
            .filter_map(|(k, (count, _))| {
                if *k > last_non_full {
                    Some(count)
                } else {
                    None
                }
            })
            .for_each(|a| *a = 0);
        self.keys.get_mut(&last_non_full).unwrap().0 += 1;
        Some(self.keys.iter().map(|(t, (size, _))| (*t, *size)).collect())
    }
}
impl<T: NumberType> From<KeyCount<T>> for KVPairIterator<T> {
    fn from(value: KeyCount<T>) -> Self {
        Self {
            keys: value.keys.into_iter().map(|(t, v)| (t, (0, v))).collect(),
        }
    }
}
#[test]
fn test_counter() {
    let c: &[(u8, usize)] = &[(8, 1), (5, 1), (7, 1), (1, 3), (3, 2), (0, 0)];
    let v: &[u8] = &[1u8, 1, 8, 5, 3, 7, 1, 3];
    let keycount = KeyCount::from_iter(v.iter().copied());
    for (c, v) in c {
        if v != keycount.keys.get(c).unwrap_or(&0) {
            panic!("Invalid count {c:?} => {v:?}")
        }
    }
}
