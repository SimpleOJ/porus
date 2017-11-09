// http://practice.geeksforgeeks.org/problems/print-the-pattern-set-1/1

#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);

    let mut t : int = default();
    scanf!(stdin, " %d", &mut t);

    for _ in 0..t {
        let mut n : int = default();
        scanf!(stdin, " %d", &mut n);

        for i in (1..n+1).rev() {
            for j in (1..n+1).rev() {
                for _ in 0..i {
                    io::print(stdout, (j," "))?;
                }
            }

            io::print(stdout, ("$",))?;
        }

        io::print(stdout, ("\n",))?;
    }

    Ok(())
}
