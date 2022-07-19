pub fn run() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5.0, y: 10 };

    let p4 = p1.mixup(p2);

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
        //Type can be ordered and copied (traits) so that they can be compared(int , chars etc)
        //generic types mentioned by <T> after function name
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
}
