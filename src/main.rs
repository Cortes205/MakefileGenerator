use std::io::Write;

/**
* Compiler flags when using the argument '-default'
*/
const DEFAULT_FLAGS : &str = " -std=c99 -Wall -pedantic";

/**
* Whether or not to store target files in a folder named bin
*/
const BIN : bool = true;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() > 1 && args[1] == "-h" {
		println!("help menu");
		std::process::exit(1);
	} 

	let objects : &mut Vec<Vec<String>> = &mut Vec::new();
	let sources : &mut Vec<Vec<String>> = &mut Vec::new();

	// TODO: Error handling with files
	let file = &mut std::fs::OpenOptions::new().write(true).append(true).create(true).open("makefile").unwrap();

	create_targets(args, file, objects, sources, 0);

	// TODO: Try to combine these next three loops somehow
	// Write the all command for each target file
	file.write_all(b"all:").unwrap();
	for i in 0..objects.len() {
		let mut out : String = " ${OUT".to_string();
		if i != 0 {
			out.push_str(&i.to_string());
		}
		file.write_all(out.as_bytes()).unwrap();
		file.write_all(b"}").unwrap();
	}
	file.write_all(b"\n\n").unwrap();

	// Write the target files + all source files underneath
	// TODO: Avoid writing duplicate source file compilations - Thinking HashMap
	for i in 0..objects.len() {
		let mut out : String = "${OUT".to_string();
		let mut obj : String = " ${OBJ".to_string();
		let mut flags : String = "\n\t${CFLAGS".to_string();
		let mut heads : String = " ${HEADS".to_string();

		if i != 0 {
			out.push_str(&i.to_string());
			obj.push_str(&i.to_string());
			flags.push_str(&i.to_string());
			heads.push_str(&i.to_string());
		}

		out.push_str("}");
		obj.push_str("}");
		flags.push_str("}");
		heads.push_str("}");

		file.write_all(out.as_bytes()).unwrap();
		file.write_all(b":").unwrap();
		file.write_all(obj.as_bytes()).unwrap();
		file.write_all(flags.as_bytes()).unwrap();
		file.write_all(obj.as_bytes()).unwrap();
		file.write_all(b" -o ").unwrap();
		if BIN {
			file.write_all(b"bin/").unwrap();
		}
		file.write_all(out.as_bytes()).unwrap();
		file.write_all(b"\n\n").unwrap();
			
		for j in 0..objects[i].len() {	
			file.write_all(objects[i][j].as_bytes()).unwrap();
			file.write_all(b": ").unwrap();
			file.write_all(sources[i][j].as_bytes()).unwrap();
			file.write_all(heads.as_bytes()).unwrap();
			file.write_all(flags.as_bytes()).unwrap();
			file.write_all(b" -c ").unwrap();
			file.write_all(sources[i][j].as_bytes()).unwrap();
			file.write_all(b"\n\n").unwrap();
		}

	}

	file.write_all(b"clean:\n\trm *.o").unwrap();
	for i in 0..objects.len() {
		let mut out : String = " ${OUT".to_string();
		if BIN {
			out = " bin/${OUT".to_string();
		}

		if i != 0 {
			out.push_str(&i.to_string());
		}

		out.push_str("}");
		file.write_all(out.as_bytes()).unwrap();
	}
}

/**
* Reads the command line arguments and collects necessary object and source file names for each target
* Differs each target by using recursion and the '-new' argument
* Writes all necessary variables to the makefile
* @param args Command line arguments
* @param file makefile that will be created
* @param objects 2D Vector storing object file names for the nth target
* @param sources 2D Vector storing source file names for the nth target
* @param target nth target - the target we are currently creating (0-Indexed)
*/
fn create_targets(args : Vec<String>, file : &mut std::fs::File, objects : &mut Vec<Vec<String>>, sources : &mut Vec<Vec<String>>, target : usize) {	
	if args.len() < 4 {
		println!("makegen: More arguments required for target #{} (minimum arguments: language, executable, source file(s)).\nUse 'makegen -h' for more info.", target+1);
		std::process::exit(1);
	}

	// Create new vector object for current target
	objects.push(Vec::new());
	sources.push(Vec::new());

	// Initialize variable names
	let mut obj_var : String = "OBJ".to_string();
	let mut out_file : String = "OUT".to_string();
	let mut header_files : String = "HEADS".to_string();
	let mut flags : String = "CFLAGS".to_string();

	if target != 0 {
		obj_var.push_str(&target.to_string());
    	out_file.push_str(&target.to_string());
		header_files.push_str(&target.to_string());
		flags.push_str(&target.to_string());
	}

	obj_var.push_str(" =");
    out_file.push_str(" = ");
	header_files.push_str(" =");
	flags.push_str(" = gcc");

	// Loop through arguments, collect info, create variables
	for i in 1..args.len() {
		let curr = &args[i];
		// First argument MUST be the language code
		if i == 1 {
			if curr != "-c" {
				println!("makegen: {} is not a known langauge code", curr);
				std::process::exit(1);
			}
		// Second argument MUST be the target file
		} else if i == 2 {
			out_file.push_str(curr);
		} else if i > 2 {
			// If this argument doesn't have a valid file extension nor is a flag/command
			if curr[curr.len()-2..curr.len()] != *".c" && curr[curr.len()-2..curr.len()] != *".h" && curr.chars().nth(0) != Some('-') {
				println!("makegen: {} has an invalid file type - ensure source files have the extension .c or .h", curr);
				std::process::exit(1);
			} else {
				if curr.chars().nth(0) == Some('-') {
					// New command passes the arguments from new onwards - to be treated for separate target
					if curr == "-new" {
						// Ensure a source file was provided for previous target before proceeding
						if sources[target].len() == 0 {
							println!("makegen: no source files given for target #{}", target+1);
							std::process::exit(1);
						}
						create_targets(args[i..args.len()].to_vec(), file, objects, sources, target+1);
						break;
					// Default flags
					} else if curr == "-default" {
						flags.push_str(DEFAULT_FLAGS);
					} else {
						flags.push_str(" ");		
						flags.push_str(curr);
					}
				} else if curr[curr.len()-2..curr.len()] == *".c" {
					// Assign current source file to current target
					sources[target].push(curr.to_string());

					// Omit folder names for object file name
					let mut start = 0;
					for i in (0..curr.len()).rev() {
						if curr.chars().nth(i) == Some('/') {
							start = i+1;
						}
					}

					// Format individual object name and object list variable
					let mut obj_name : String = curr[start..curr.len()-2].to_string();
					obj_name.push_str(".o");
					obj_var.push_str(" ");
					obj_var.push_str(&obj_name);

					// Assign current object to current target
					objects[target].push(obj_name);
				} else if curr[curr.len()-2..curr.len()] == *".h" {
					// Format header file variable
					header_files.push_str(" ");
					header_files.push_str(curr);
				}
			}
		}

		/* The loop only makes it to the end if there are no more new targets
		(if we've finished collecting all data for targets) */
		if i == args.len()-1 {
			if sources[target].len() == 0 {
				println!("makegen: no source files given for target #{}", target+1);
				std::process::exit(1);
			}
			// Clear the file now that we have all necessary data
			file.set_len(0).unwrap();
		}	
	}

	// Write out all the variables
	obj_var.push_str("\n");
	file.write_all(obj_var.as_bytes()).unwrap();
	out_file.push_str("\n");
	file.write_all(out_file.as_bytes()).unwrap();
	if header_files != "HEADS =" {
		header_files.push_str("\n");
		file.write_all(header_files.as_bytes()).unwrap();
	}
	file.write_all(flags.as_bytes()).unwrap();
	file.write_all(b"\n\n").unwrap();

}
