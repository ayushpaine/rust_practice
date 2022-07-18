pub fn run() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    {
        let v2 = vec![1, 2, 3]; //at the end of this block v2 will be dropped(end of scope)
    }

    let mut v2 = vec![1, 2, 3, 4];
    let third = &v2[2]; //out of size gives compile time error for arrays but runtime error for vectors
                        //as vectors' size isnt known at compile time
    println!("third {}", third);

    match v2.get(2) {
        Some(third) => println!("{}", third),
        None => println!("no element"),
    }

    for i in &mut v2 {
        println!("{}", i);
        *i += 50;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("red")),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("not int"),
    };
}
