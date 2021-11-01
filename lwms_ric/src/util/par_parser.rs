pub fn parse_args(args: &[String]) -> Result<(&str, &str), &str> {
    if args.len() != 3 {
        return Err("Incorrect number of arguments - please provide 2 arguments: module name and script name");
    }
    let side_module_name = &args[1] ;
    let main_module_name = &args[2] ;

    Ok((side_module_name, main_module_name))
}