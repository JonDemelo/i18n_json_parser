use std::env;
use std::fs::read_to_string;
use std::fs::write;
use std::process;

// This is a Rust-based JSON parser for i18n files translated using Google Translate.
// It converts the naively translated text back into usable JSON for i18n services.

// This is not intended to be a complete tool for all use cases and whatnot -- I'm merely hacking together something quick for my personal use case

// See README.md for more information.

// If you don't want these errors to be "naively" handled, i.e. just give you the file and you can clean it up in post, change this to false.
// Otherwise, see fn clean_google_translate_errors
const HANDLE_ERRORS: bool = true;

fn read_in_file_lines(file_path: String) -> Vec<String> {
    read_to_string(file_path)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn write_to_file(file_path: String, content: String) {
    write(file_path, content).expect("Unable to write file");
}

fn clean_google_translate_errors(file_line: String) -> String {
    if !HANDLE_ERRORS {
        return file_line;
    }

    let mut working_line: String = file_line;
    working_line = String::from(working_line.trim());
    //
    // If ": " is not in line, insert where first space is. This is not a complete solution, but best guess under circumstances.
    // This is because there's a chance that the left-side is translated into multiple words, perhaps along with the right side.
    // This would leave a situation where...
    // "word secondword actualaftersplit perhapsthis too",
    // which should've been...
    // "word secondword": "actualaftersplit perhapsthis too",
    // or maybe it was suppose to be ...
    // "word": "secondword actualaftersplit perhapsthis too",
    // Only an actual human-reader with language context could know. So just pop it after first space and let the post-computation reviews handle errors.
    if !working_line.contains(": ") && !(working_line.eq("{") || working_line.eq("}")) {
        working_line = working_line.replacen(" ", ": ", 1);
    }

    // Now, this fix checks if the 2nd to last character is a punctuation AND the third to last character is a ", if this is the case, then swap their positions. This assumes EOL-1 is a ,
    // i.e. "... "!, -> "... !",
    // Naive implementation as it only covers a few of the most common punctuations, latin-originating and greek, and only l->r language coverage.
    let punctuation = vec!['!', '?', ',', '.', '¿', ';', ':', '·', '¡'];

    if working_line.len() > 2 {
        let start_pos = working_line.chars().count() - 3;
        let end_pos = working_line.chars().count() - 2;

        let second_to_last: char = working_line.chars().nth(end_pos).unwrap();
        let third_to_last: char = working_line.chars().nth(start_pos).unwrap();

        if third_to_last == '"' && punctuation.contains(&second_to_last) {
            let swapped: String = [second_to_last.to_string(), third_to_last.to_string()].join("");
            working_line.replace_range(start_pos..end_pos, &swapped);
        }
    }
    return working_line;
}

fn main() {
    let middle_path: &str = "./src/translations/";
    let source_path: String = [middle_path, "source/"].join("");
    let dest_path: String = [middle_path, "dest/"].join("");
    let mut base_filename: String = String::from(middle_path).to_owned();

    let mut has_read_first_arg: bool = false;
    let mut has_read_second_arg: bool = false;

    let args: env::Args = env::args();
    let mut filenames: Vec<(String, String)> = Vec::new();

    if args.len() < 3 {
        println!("\nRequired one base argument, and at least one source arguments.\n");
        process::exit(1); // If the number of arguments is too short to function, end program.
    }

    for arg in args {
        if !has_read_first_arg {
            has_read_first_arg = true; // ignore program arg
        } else if !has_read_second_arg {
            has_read_second_arg = true; // base_filename arg
            if arg.ends_with(".json") {
                base_filename.push_str(&arg);
            } else {
                println!("\nBase argument must be a JSON file.\n");
                process::exit(1); // Files must be JSONs.
            }
        } else {
            if arg.ends_with(".json") {
                let source_path_complete: String = [source_path.clone(), arg.clone()].join("");
                let dest_path_complete: String = [dest_path.clone(), arg.clone()].join("");
                filenames.push((source_path_complete, dest_path_complete));
            }
        }
    }

    // scrap structure and params from base filename.
    let base_file_lines: Vec<String> = read_in_file_lines(base_filename);
    let mut cleaned_base_file_lines: Vec<String> = Vec::new();

    for line in base_file_lines {
        let error_fixed_line: String = clean_google_translate_errors(line);
        let split_string: Vec<&str> = error_fixed_line.split(":").collect();
        let param_string: String = split_string[0].replace(" ", "");
        cleaned_base_file_lines.push(param_string);
    }

    // For each of the provided argument filenames, already prepped in their full paths, both source and eventual destination.
    for file_tuple in filenames {
        let mut new_file_content: String = String::new();
        let copy_base_content = cleaned_base_file_lines.clone();
        let source_file_content: Vec<String> = read_in_file_lines(file_tuple.0);
        let mut cleaned_source_content: Vec<String> = Vec::new();

        for line in source_file_content {
            let mut postfix: String = String::new();

            match line.split_once(':') {
                Some((_, post)) => {
                    postfix = String::from(post);
                }
                None => {}
            }

            cleaned_source_content.push(postfix);
        }

        for (i, line) in cleaned_source_content.iter().enumerate() {
            if i < copy_base_content.len() {
                let mut combo: String = String::new();
                combo.push_str(&copy_base_content[i]);
                if !combo.is_empty() && !(combo.eq("{") || combo.eq("}")) {
                    combo.push_str(": ");
                }
                combo.push_str(line);
                combo = combo.replace("  ", " ");
                combo.push('\n');

                new_file_content.push_str(&combo);
            }
        }

        write_to_file(file_tuple.1, new_file_content);
    }
}
