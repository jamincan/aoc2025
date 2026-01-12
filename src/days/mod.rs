use paste::paste;

macro_rules! days {
    ($($n:expr),*) => {
        $(
            paste! { pub mod [<day $n>]; }
        )*
        paste! {
            pub const COUNT: usize = [$($n),*].len();
            pub const SOLUTIONS: [[fn(); 2]; COUNT] = [
                $([ [<day $n>]::part1, [<day $n>]::part2 ], )*
            ];
        }
    };
}

days!(1, 2, 3, 4, 5, 6, 7, 8);
