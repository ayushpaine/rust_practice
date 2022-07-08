/*
primitive types:
->integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (their memory size in bits)
->floats: f32, f64
->boolean: true, false(bool)
->characters(char)
->tuples(lists)
->array(fixed size vectors have variable)
*/

/*rust is statically typed lang so it must know the types of all variables at compile time however compiler can
infer types based on value and usage
*/

//add underscore to variable at start to remove not used warning from compiler
pub fn run() {
    //default i32
    let _i = 1;

    //default f64
    let _j = 2.5;

    //explicit
    let _k: i64 = 2983121;

    //max size
    println!("Max i64 :{}", std::i64::MAX);

    //bool
    let is_active: bool = true;

    //bool from extension
    let is_greater: bool = 10 > 7;

    let a1: char = 'a';
    let emoji: char = '\u{1F600}';

    println!("{:?}", (_i, _j, _k, is_active, is_greater, a1, emoji));
}
