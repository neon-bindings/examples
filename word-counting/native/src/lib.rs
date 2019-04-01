use neon::prelude::*;
use neon::register_module;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::str::from_utf8;

fn lines(corpus: &str) -> Vec<&str> {
    corpus
        .lines()
        .map(|line| line.splitn(4, ',').nth(3).unwrap().trim())
        .collect()
}

fn matches(word: &str, search: &str) -> bool {
    let mut search = search.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match search.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return search.next().is_none();
}

fn wc_line(line: &str, search: &str) -> i32 {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, search) {
            total += 1;
        }
    }
    total
}

// Also valid, with comparable performance:

/*
fn wc_line(line: &str, search: &str) -> i32 {
    line.split(' ')
        .filter(|word| matches(word, search))
        .fold(0, |sum, _| sum + 1)
}
*/

fn _wc_sequential(lines: &Vec<&str>, search: &str) -> i32 {
    lines
        .into_iter()
        .map(|line| wc_line(line, search))
        .fold(0, |sum, line| sum + line)
}

fn wc_parallel(lines: &Vec<&str>, search: &str) -> i32 {
    lines
        .into_par_iter()
        .map(|line| wc_line(line, search))
        .sum()
}

fn search(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let buffer: Handle<JsBuffer> = cx.argument(0)?;
    let string = cx.argument::<JsString>(1)?.value();
    let search = &string[..];
    let total = cx.borrow(&buffer, |data| {
        let corpus = from_utf8(data.as_slice()).ok().unwrap();
        wc_parallel(&lines(corpus), search)
    });
    Ok(cx.number(total))
}

register_module!(mut m, { m.export_function("search", search) });
