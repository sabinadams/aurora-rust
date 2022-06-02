mod util;
mod vars;
fn main() {
    // Read the config file
    let aurora_config = util::file::read_aurora_config().unwrap_or_else(|_err| {
        eprintln!("Could not find or parse the {}", vars::CONFIG_PATH);
        std::process::exit(1)
    });

    // Get all the files as strings & validate them
    let schemas = util::file::read_all_schemas(aurora_config.files);
    if schemas.len() == 0 {
        eprintln!("No schemas found");
        std::process::exit(0)
    }

    
    util::datamodel::consolidate_schemas(
        schemas.iter().map(|x| datamodel::parse_schema(x).unwrap()).collect()
    );

}
