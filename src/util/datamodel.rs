use datamodel::{ValidatedDatamodel};
use crate::util;
pub fn validate_schema(
    schema: String
) -> Result<ValidatedDatamodel, datamodel::diagnostics::Diagnostics> {
    datamodel::parse_datamodel(&schema)
}

pub fn consolidate_schemas(
    schemas: Vec<(datamodel::Configuration, datamodel::dml::Datamodel)>,
) -> (datamodel::Configuration, datamodel::dml::Datamodel) {
    let mut builder = util::builder::Builder::new();
    for schema in schemas {
        for datasource in schema.0.datasources {
            builder.add_datasource(datasource);
        }
    }

    builder.print();

    builder.render()
}

