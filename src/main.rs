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
			// TODO: Header file flag

			if curr[curr.len()-2..curr.len()] != *".c" {
				println!("makegen: {} is an invalid file type - ensure source files have the extension .c", curr);
				std::process::exit(1);
			}
		}
	}
	
	let mut objects : Vec<String> = Vec::new();
	let mut sources : Vec<String> = Vec::new();
	let mut obj_var : String = "OBJ = ".to_string();
	let mut out_file : String = "OUT = ".to_string();
	let mut prev : &str = "";
	for i in 2..args.len() {
		let curr = &args[i];
		if i == 2 {
			out_file.push_str(curr);
		} else if curr == "-head" {
			prev = curr;
			continue;
		} else if prev == "-head" {
			// TODO: After implementing -head flag as argument
		} else {
			sources.push(curr.to_string());
			let mut obj_name : String = curr[..curr.len()-2].to_string();
			obj_name.push_str(".o");
			obj_var.push_str(&obj_name);
			obj_var.push_str(" ");
			objects.push(obj_name);
		}
	} 

	// TODO: Error handling with files
	let mut file = std::fs::OpenOptions::new().write(true).append(true).create(true).open("makefile").unwrap();
	file.set_len(0).unwrap();

	obj_var.push_str("\n");
	file.write_all(obj_var.as_bytes()).unwrap();
	out_file.push_str("\n");
	file.write_all(out_file.as_bytes()).unwrap();	

	file.write_all(b"CFLAGS = gcc -std=c99 -Wall -pedantic\n\n${OUT}: ${OBJ}\n\t${CFLAGS} ${OBJ} -o bin/${OUT}\n\n").unwrap();
	
	for i in 0..objects.len() {	
		file.write_all(objects[i].as_bytes()).unwrap();
		file.write_all(b": ").unwrap();
		file.write_all(sources[i].as_bytes()).unwrap();
		file.write_all(b"\n\t${CFLAGS} -c ").unwrap();
		file.write_all(sources[i].as_bytes()).unwrap();
		file.write_all(b"\n\n").unwrap();
	}

	file.write_all(b"clean:\n\trm *.o bin/${OUT}").unwrap();	
}
