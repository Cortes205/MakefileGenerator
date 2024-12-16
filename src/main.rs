use std::io::Write;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() > 1 && args[1] == "-h" {
		println!("help menu");
		std::process::exit(1);
	} else if args.len() < 4 {
		println!("makegen: More arguments required (minimum arguments: language, executable, source file(s)).\nRun with -h for more info.");
		std::process::exit(1);
	}

	for i in 1..args.len() {
		let curr = &args[i];
		if i == 1 {
			if curr != "-c" {
				println!("makegen: {} is not a known langauge code", curr);
				std::process::exit(1);
			}
		} else if i > 2 {
			if curr[curr.len()-2..curr.len()] != *".c" && curr[curr.len()-2..curr.len()] != *".h" && curr.chars().nth(0) != Some('-') {
				println!("makegen: {} is an invalid file type - ensure source files have the extension .c or .h", curr);
				std::process::exit(1);
			}
		}
	}
	
	let mut objects : Vec<String> = Vec::new();
	let mut sources : Vec<String> = Vec::new();
	let mut obj_var : String = "OBJ =".to_string();
	let mut out_file : String = "OUT = ".to_string();
	let mut header_files : String = "HEADS =".to_string();
	let mut flags : String = "CFLAGS = gcc".to_string();
	for i in 2..args.len() {
		let curr = &args[i];
		if i == 2 {
			out_file.push_str(curr);
		} else {
			if curr.chars().nth(0) == Some('-') {
				if curr == "-default" {
					flags.push_str(" -std=c99 -Wall -pedantic");
				} else {
					flags.push_str(" ");		
					flags.push_str(curr);
				}
			} else if curr[curr.len()-2..curr.len()] == *".c" {
				sources.push(curr.to_string());
				let mut start = 0;
				for i in (0..curr.len()).rev() {
					if curr.chars().nth(i) == Some('/') {
						start = i+1;
					}
				}
				let mut obj_name : String = curr[start..curr.len()-2].to_string();
				obj_name.push_str(".o");
				obj_var.push_str(" ");
				obj_var.push_str(&obj_name);
				objects.push(obj_name);
			} else if curr[curr.len()-2..curr.len()] == *".h" {
				header_files.push_str(" ");
				header_files.push_str(curr);
			}
		}
	} 

	if sources.len() == 0 {
		println!("makegen: no source files given");
		std::process::exit(1);
	}

	// TODO: Error handling with files
	let mut file = std::fs::OpenOptions::new().write(true).append(true).create(true).open("makefile").unwrap();
	file.set_len(0).unwrap();
	
	obj_var.push_str("\n");
	file.write_all(obj_var.as_bytes()).unwrap();
	out_file.push_str("\n");
	file.write_all(out_file.as_bytes()).unwrap();
	if header_files != "HEADS =" {
		header_files.push_str("\n");
		file.write_all(header_files.as_bytes()).unwrap();
	}
	file.write_all(flags.as_bytes()).unwrap();

	file.write_all(b"\n\n${OUT}: ${OBJ}\n\t${CFLAGS} ${OBJ} -o bin/${OUT}\n\n").unwrap();
	
	for i in 0..objects.len() {	
		file.write_all(objects[i].as_bytes()).unwrap();
		file.write_all(b": ").unwrap();
		file.write_all(sources[i].as_bytes()).unwrap();
		if header_files != "HEADS =" {
			file.write_all(b" ${HEADS}").unwrap();
		}
		file.write_all(b"\n\t${CFLAGS} -c ").unwrap();
		file.write_all(sources[i].as_bytes()).unwrap();
		file.write_all(b"\n\n").unwrap();
	}

	file.write_all(b"clean:\n\trm *.o bin/${OUT}").unwrap();	
}
