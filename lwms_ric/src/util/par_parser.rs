pub fn parse_args<'a>(args: &'a [String], file_prefix: &'a str, module_names: &'a mut Vec<String>) -> Result<(), &'a str> {
    if args.len() < 2 {
        return Err("Incorrect number of arguments - please provide number of files and their names");
    }

    let expected_par: usize = args[1].parse::<usize>().unwrap();

    if args.len() < (expected_par + 2) {
            return Err("Incorrect number of arguments - please provide number of files and their names");
    }

    let mut it = 0;
    while it < expected_par {
      module_names.push(format!("{}/target/debug/{}", file_prefix, args[it + 2]));
      it = it + 1;
    }

    Ok(())
}