use std::{fs::File, io::Write, path::Path};
use std::error::Error;
use std::io::{BufRead, BufReader};

fn main() {
    write_num_to_file("f.txt");
    match read_num_from_file("f.txt") {
        Err(err) => {
            panic!("{:?}", err)
        }
        Ok(_) => {}
    }
}

fn read_num_from_file(p: &str) -> Result<(), Box<dyn Error>> {
    let mut v = Vec::new();
    let path = Path::new(p);
    let file = File::open(&path)?;

    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        if let Ok(line) = line {
            let tokens: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            for token in tokens {
                v.push(token);
            }
        } else {
            panic!("read err")
        }

    }

    v.reverse();

    let mut file = File::create("f2.txt")?;
    for i in 0..v.len() {
        file.write(v[i].as_bytes())?;
        if (i+1) % 3 == 0 {
            file.write("\n".as_bytes())?;
        } else {
            file.write(",".as_bytes())?;
        }
    }

    

    Ok(())
}


fn write_num_to_file(p: &str) {
    let path = Path::new(p);

    let mut file = match File::create(&path) {
        Err(err) => panic!("{:?}", err),
        Ok(file) => file,
    };

    for i in 1..100 {
        match file.write(i.to_string().as_bytes()) {
            Err(err) => {
                panic!("{:?}", err)
            }
            Ok(_) => {}
        }

        if i % 3 == 0 {
            match file.write("\n".as_bytes()) {
                Err(err) => {
                    panic!("{:?}", err)
                }
                Ok(_) => {}
            }
        } else {
            match file.write(",".as_bytes()) {
                Err(err) => {
                    panic!("{:?}", err)
                }
                Ok(_) => {}
            }
        }
    }
}