#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut i : isize = 1;
    loop {
        let mut x : isize = default();
        scanf!(stdin, " %d", &mut x);
        if x == 0 {
            break;
        }
        printf!(stdout, "Case %d: %d\n", i, x);
        i += 1;
    }
}