
trait Handler {
    fn add(&self) -> Box<Fn(i32, i32) -> i32>;
}

#[derive(Debug)]
struct HandlerImpl1 {
    i1: i32,
}

impl Handler for HandlerImpl1 {
    fn add(&self) -> Box<Fn(i32, i32) -> i32> {
        let i = self.i1;
        Box::new(move |x, y| x + y + i)
    }
}

#[derive(Debug)]
struct HandlerImpl2 {
    i2: i32,
}
impl Handler for HandlerImpl2 {
    fn add(&self) -> Box<Fn(i32, i32) -> i32> {
        Box::new(move |x, y| x + y + 5)
    }
}

type HandlerType = fn(i32, i32) -> i32;

fn add1(i: i32, j: i32) -> i32 {
    i + j
}

#[test]
fn test_demo_1() {
    let tmp: HandlerType = add1;
    let hnd = HandlerImpl1 { i1: 2 };
    assert_eq!(2, tmp(1, 1));
    let tmp = hnd.add();
    assert_eq!(5, tmp(1, 2));

    let tmp: &Handler = &HandlerImpl2 { i2: 3 };
    assert_eq!(7, tmp.add()(1, 1));

}
