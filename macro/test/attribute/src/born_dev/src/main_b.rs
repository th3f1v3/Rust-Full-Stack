// https://github.com/rust-lang/rust/blob/master/src/libstd/macros.rs
// https://www.google.com/search?client=firefox-b-d&q=what+is+token+tree+in+rust
// https://doc.rust-lang.org/rust-by-example/trait/derive.html

// #[derive(Debug)]
macro_rules! base {
    (struct $name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        struct $name {
            id: u64,
            meta: f32,
            $( $field: $ty ),*
        }
    };
    (struct $name:ident { $( $field:ident: $ty:ty ),* $(,)* }, $($arg:tt)+) => {
        #[derive($($arg)+)] // How to use it more?
        // #[derive($($arg)*)] // How to use it dynamically here?, #[derive(Debug)]
        struct $name {
            id: u64,
            meta: f32,
            $( $field: $ty ),*
        }
    };
}

base!(struct Example0 {
    x: f32,
    y: f32,
});


base!(struct Example1 {
    x: f32,
    y: f32,
}, Debug);

// Test impl here.

impl Example1 {
    fn sum_f32(&self) -> f32 {
        &self.x + &self.y
    }
}

base!(struct Example2 {
    a: f32,
    b: f32,
}, Debug, Clone, PartialEq);

// Test function here.
fn sum_f32(a: &f32, b: &f32) -> f32 {
   let result = a + b;
   result
}

fn main() {
    let example0 = Example0 {
        id: 1,
        meta: 1.0,
        x: 1.0,
        y: 1.0,
    };

    println!("{}", example0.id);
    println!("{}", example0.meta);
    println!("{}", example0.x);
    println!("{}", example0.y);
    // println!("{:#?}", example0);

    let example1 = Example1 {
        id: 1,
        meta: 1.0,
        x: 1.0,
        y: 1.0,
    };

    let example2 = Example2 {
        id: 1,
        meta: 1.0,
        a: 1.0,
        b: 1.0,
    };
    println!("{}", example1.id);
    println!("{:#?}", example1);
    println!("{}", example2.id);
    println!("{:#?}", example2);

    println!("{}", &example1.sum_f32());
    println!("{}", sum_f32(&example2.a, &example2.b));

    assert_eq!(example2, example2.clone());
}
