use std::{
    cmp::min,
    env::args,
    usize,
    collections::HashSet,
    hash::Hash,
};
use codepage_437::{IntoCp437, BorrowFromCp437, CP437_CONTROL};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn utf8_to_cp437(utf8: String) -> Vec<u8> {
    let result = utf8.into_cp437(&CP437_CONTROL);
    match result {
        Ok(vec_u8) => {
           vec_u8
        },
        Err(cp437_err) => {
           let bad_arg = cp437_err.into_string();
           panic!("Argument \"{}\" is not representable in Code Page 437", bad_arg)
        }
    }
}

fn main() {
    let input = args()
        .skip(1) // Skip program name
        .map(utf8_to_cp437) // Convert utf-8 to Codepage 437 (extended ASCII)
        .collect::<Vec<Vec<u8>>>(); // Collect into vector


    // The max length of the answers is the min length among inputs
    let max_len = input.iter()
        .fold(usize::MAX, |x,y| {
            min(x,y.len())
        });

    let mut answer_len = 1usize;
    loop {
        if answer_len > max_len {
            panic!("No equal-length abbreviation is possible for the given inputs.");
        }
        let slices_up_to_len = input.iter()
            .map(|x| { &x[0..answer_len] });

        if has_unique_elements(slices_up_to_len) {
            break;
        }
        answer_len += 1;
    };

    for arg in input {
        let out = String::borrow_from_cp437(&arg[0..answer_len], &CP437_CONTROL);
        print!("{} ", out);
    }
}
