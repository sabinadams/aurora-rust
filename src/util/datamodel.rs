use crate::util;
use datamodel::ValidatedDatamodel;

/// Runs a datamodel string through `parse_model`, which will throw an error if the model is invalid.
pub fn validate_schema(
    schema: String,
) -> Result<ValidatedDatamodel, datamodel::diagnostics::Diagnostics> {
    datamodel::parse_datamodel(&schema)
}

/// Takes all of your schemas and merges things together
pub fn consolidate_schemas(
    schemas: Vec<(datamodel::Configuration, datamodel::dml::Datamodel)>,
) -> (datamodel::Configuration, datamodel::dml::Datamodel) {
    let mut builder = util::builder::Builder::new();
    for schema in schemas {
        for datasource in schema.0.datasources {
            builder
                .add_datasource(datasource)
                .unwrap_or_else(|_| std::process::exit(0));
        }
    }

    builder.print();

    builder.render()
}
