use middle_lib;
use std::sync::Arc;
use trait_lib::Check;

struct Final {
    val: i32,
}

impl trait_lib::Check for Final {
    fn check(&self, arg: i32) -> bool {
        self.val + 2 > arg
    }
}

fn make_trait(val: i32) -> Arc<Box<dyn Check>> {
    Arc::new(Box::new(Final { val }))
}

fn main() {
    let fin = make_trait(12);
    println!("{}", middle_lib::check_trait(fin, 10));
}
