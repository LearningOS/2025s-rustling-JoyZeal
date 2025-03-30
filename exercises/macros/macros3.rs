// 正确解法
pub mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 在模块内部重新导出宏
    pub use crate::my_macro;
}

fn main() {
    macros::my_macro!(); // 现在可以通过模块路径调用
}