use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str;

    // if no arguments, use test1.txt in the current directory
    if args.len() < 2 {
        file_path = "test1.txt";
    }
    else {
        file_path = &args[1];
    }
    process_file(file_path);
}

fn process_file(path: &str) {
    let output_file_name = "output.txt";

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", &path, &why),
        Ok(file) => file,
    };

    let out_file = match File::create(output_file_name) {
        Err(why) => panic!("couldn't create output file {}: {}", &path, &why),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(out_file);

    // BufReader will return an iterator over the lines in the file
    // Loop through the lines and extract the digits
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for l in lines {
        let l2 = l.unwrap();

        // Match on any digits in the string and collect them into a vector
        let nums: Vec<_> = l2
            .match_indices(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            .map(|(i, _)| i)
            .collect();

        if nums.len() == 0 {
            // This shouldn't happen according to spec, but just in case
            //println!("--");
        } else {
            let i2 = nums[nums.len() - 1];

            // build a string with the first & last digits in the line
            let mut line_str = l2.chars().nth(nums[0]).unwrap().to_string();
            line_str.push_str(&l2.chars().nth(i2).unwrap().to_string());

            match writer.write_fmt(format_args!("{}\n",line_str)) {
                Ok(_) => {},
                Err(why) => panic!("couldn't write to output file {}: {}", &path, &why),
            }
        }
    }

    // print path to output file using OS delimiters
    let mut cd = env::current_dir().unwrap();
    cd = cd.join(output_file_name);

    println!("{}", &cd.to_str().unwrap());
}
