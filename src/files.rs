// Intro to files
use rand;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn simulate() {
    let result = OpenOptions::new()
        .write(true)
        .read(true)
        .open("./trial.txt");

    return match result {
        Ok(file) => {
            println!("File read");
            match file.try_clone() {
                Ok(file) => {
                    let file_reader = BufReader::new(file);
                    let file_reader = file_reader.lines();

                    for line in file_reader {
                        match line {
                            Ok(line) => println!("{line}"),
                            Err(_) => print!("Reached end"),
                        }
                    }
                }
                Err(_) => todo!(),
            }

            let mut file_writer = BufWriter::new(file);
            for _ in 0..10 {
                let mut random = rand::random::<i32>().to_string();
                random.push('\n');
                let _ = file_writer.write(random.as_bytes());
            }
            let res = file_writer.flush();
            match res {
                Ok(_) => {
                    println!("Saved file")
                }
                Err(err) => {
                    println!("error writing to file {err:?}")
                }
            }
        }
        Err(err) => {
            println!("Error opening file \n{err}")
        }
    };
}
