use vstd::prelude::*;

verus! {
fn baz()
ensures 2 + 2 == 3
{
    assert(2 + 2 == 5);
}

fn abc()
ensures 2 + 2 == 3
{
    assert(4 + 4 == 10);
}

fn def()
ensures 2 + 2 == 7
{
    assert(4 + 4 == 8);
}

fn main() {}

}
