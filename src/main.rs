mod base;
mod utils;

use base::*;
use utils::*;

fn main() {
    let mut stack = Stack::new();
    stack.push(1);

    par_checker("(23*((a+b)+(b+a)/45))%4");

    let postfix = infix_to_postfix("(A+B)*C-D/E");
}
