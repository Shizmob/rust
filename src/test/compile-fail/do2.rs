// xfail-fast
// xfail-test

// This is xfail'd due to bad typecheck error messages. (There is a spurious
// "expected `bool` but but found `int`" message.)

fn f(f: fn@(int) -> bool) -> bool { f(10i) }

fn main() {
    assert do f() |i| { i == 10i } == 10i;
    //~^ ERROR: expected `bool` but found `int`
    //~^^ ERROR: expected `bool` but found `int`
}
