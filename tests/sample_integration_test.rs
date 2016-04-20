extern crate scirust;

pub fn foo() -> i32 {
    0
}

pub fn execute(f: &Fn() -> i32) -> i32 {
    f()
}

#[test]
fn test_execute_foo() {
    let ans = execute(&foo);
    assert_eq!(ans, 0);
}
