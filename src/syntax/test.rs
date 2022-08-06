fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test] // 在测试时才会编译
fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic] // 在panic的情况下才会通过测试
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}

#[test]
#[ignore = "not yet reviewed by the Q.A. team"] // 暂时跳过该测试函数
fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
}

#[cfg(test)] // 测试模块
mod add_function_tests {
    use super::add;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}