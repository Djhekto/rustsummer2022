use std::fmt::Display;

struct Pair<T> { x: T, y: T,}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }
                }

impl<T: Display + PartialOrd> Pair<T>  {
    fn cmp_display(&self) {
        if self.x >= self.y { println!("The largest member is x = {}", self.x); } 
        else { println!("The largest member is y = {}", self.y); }
                          }
                                        }
//-------------------------------------------------------------------------//
fn main()
{
let asfd = Pair { x: 0.1, y: 1000000000000000.0, };
//asfd.new();  <- the following associated functions; to be used as methods, functions must have a `self` parameter
asfd.cmp_display();
let asfd = Pair { x: 'a', y: 'b', };
asfd.cmp_display();
let asfd = Pair::new(1,2);  // <-  tak
asfd.cmp_display();
}