use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::io::{BufReader, BufWriter};
use std::io;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


fn main() {
    let before: Instant = Instant::now();
    let lines: Vec<String> = read_file("divvy-tripdata.csv");
    let new_lines: Vec<String> = covert_lines_to_tabs(lines);
    write_iter_to_file(new_lines);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn read_file(path: &str) -> Vec<String> {
    let contents: io::Result<Vec<String>> = BufReader::new(File::open(path).expect("can't open file")).lines().collect();
    return contents.expect("something went wrong reading the file");
}

fn covert_lines_to_tabs(lines: Vec<String>) -> Vec<String> {
    lines.par_iter().map(|line: &String| -> String {
                                                    line.replace(",", "\t")
                                                }
                                            ).collect::<Vec<String>>()
}

fn write_iter_to_file(lines: Vec<String>) {
    let mut writer: BufWriter<File> = BufWriter::new(File::create("divvy-biketrip.tsv").expect("problem with file"));
    writer.write(lines.join("\n").as_bytes()).expect("problem writing lines");
}
