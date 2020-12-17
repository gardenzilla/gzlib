use crate::id::IdKind;

pub fn make_id(u: u64, id_kind: IdKind) -> u64 {
    match id_kind {
        IdKind::LuhnOne => (u * 10 + calc_check(u) as u64),
        IdKind::LuhnTwo => (u * 10 + calc_check(u) as u64) * 10 + (9 - calc_check(u) as u64),
    }
}

// Luhn algorythm
// based on https://en.wikipedia.org/wiki/Luhn_algorithm
fn calc_check(u: u64) -> u8 {
    match 10 - (calc(u, 1) % 10) {
        x if x != 10 => x,
        _ => 0,
    }
}

fn calc(n: u64, i: u32) -> u8 {
    let digit = n % 10;
    // if odd
    let res = match i % 2 {
        0 => digit,
        _ => match digit * 2 {
            x if x > 9 => x - 9,
            x => x,
        },
    };
    // Should we continue?
    let left = n / 10;
    if left > 0 {
        // Then continute calling C but with i + 1
        return calc(left, i + 1) + res as u8;
    }
    res as u8
}

pub fn is_valid(n: u64, id_kind: IdKind) -> bool {
    match id_kind {
        IdKind::LuhnOne => {
            let check = calc_check(n / 10) as u64;
            (n % 10) == check
        }
        IdKind::LuhnTwo => {
            let check = calc_check(n / 100) as u64;
            ((n % 10) == 9 - check) && ((n % 100) / 10 == check)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, hash::Hash};

    use super::*;

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    fn test_all_unique() {
        // Test if the first 50_000 items
        // are unique
        let d = (1..50_000)
            .into_iter()
            .map(|i| make_id(i, IdKind::LuhnTwo))
            .collect::<Vec<u64>>();
        assert!(has_unique_elements(d));
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(56527, IdKind::LuhnTwo), true);
        assert_eq!(is_valid(56543, IdKind::LuhnTwo), false);
        assert_eq!(is_valid(56532, IdKind::LuhnTwo), false);
        assert_eq!(is_valid(56609, IdKind::LuhnTwo), true);
        assert_eq!(is_valid(5, IdKind::LuhnOne), true);
    }
}
