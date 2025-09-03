use vstd::prelude::*;

verus! {
fn baz()
ensures 2 + 2 == 3
{
    assert(2 + 2 == 4);
}

fn main() {}

}