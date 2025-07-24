fn main() {
    // 1. panic的用法
    panic!("Error");

    // 2. 使用参数来打印遇到的错误的函数栈信息
    // RUST_BACKTRACE=1
    // 还可以打印出完全的栈信息
    // Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}
