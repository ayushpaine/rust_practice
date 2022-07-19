use core::panic;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

#[allow(unreachable_code)]

pub fn run() {
    {
        panic!("yeet"); //immediately quit program and prints error message
    }
    /*enum Result<T, E> {
        Ok(T),
        Err(E),
    }*/
    //similar to options enums already in scope

    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("problem creating file: {:#?}", e),
                },
                other_kind => panic!("problem opening file: {:#?}", other_kind),
            },
        };
    }

    {
        let f = File::open("hello.txt").unwrap(); //opens file if exists or panics
        let f = File::open("hello.txt").expect("error"); //opens file if exists or panics and displays error
    }

    {
        //error propagation
        //in general result enum and error proagation should be used and panic should be used where recovering from the program cant be done

        //custom types for validation can also be introduced
        fn read_username() -> Result<String, io::Error> {
            {
                let mut s = String::new();
                let mut f = File::open("hello.txt")?;
                //similar to unwrap or expect methods
                f.read_to_string(&mut s)?;
                return Ok(s);
                {
                    let mut s = String::new();
                    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
                    //similar to unwrap or expect methods
                    return Ok(s);
                }
            }
            {
                let f = File::open("hello.txt");
                let mut f = match f {
                    Ok(file) => file,
                    Err(e) => return Err(e),
                };

                let mut s = String::new();

                match f.read_to_string(&mut s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }
            }
        }
    }
}
