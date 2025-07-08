// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro; // 👈 关键：将宏导出到父模块
}

fn main() {
    macros::my_macro!();
}
