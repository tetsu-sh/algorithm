#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

// #[allow(dead_code)]
// #[allow(clippy::collapsible_if)]
// #[allow(clippy::needless_range_loop)]
// #[allow(clippy::collapsible_else_if)]
fn main() {
    input! {
        table:[Chars;9]
    }
    let mut count = 0;
    for i in 0..9 {
        for j in 0..9 {
            if table[i][j] == '#' {
                for a in 1..=8 {
                    for b in a..(8 - a) {
                        if i + a >= 9
                            || j + a >= 9
                            || i + b >= 9
                            || j + b >= 9
                            || i < a
                            || j < a
                            || i < b
                            || j < b
                        {
                            break;
                        }
                        if table[i + a][j + b] == '#'
                            && table[i - a][j + b] == '#'
                            && table[i + a + b][j] == '#'
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}
