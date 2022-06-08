use std::process::exit;

use crate::builder;
use datamodel::{ValidatedDatamodel, ast::{SchemaAst, Top}};

/// Runs a datamodel string through `parse_model`, which will throw an error if the model is invalid.
pub fn validate_schema(
    schema: String,
) -> Result<ValidatedDatamodel, datamodel::diagnostics::Diagnostics> {
    datamodel::parse_datamodel(&schema)
}

/// Takes all of your schemas and merges things together
pub fn consolidate_schemas(
    schemas: Vec<(std::path::PathBuf, SchemaAst)>,
) {
    let mut builder = builder::Builder::new();
    for schema in schemas {
        schema.1.iter_tops().for_each(|top| {
            match top.1 {
                Top::Source(source) => {
                    builder.add_datasource(source, schema.0.clone())
                        .unwrap_or_else(|_| {
                            exit(0)
                        });
                }
                Top::Generator(generator) => {
                    builder.add_generator(generator, schema.0.clone())
                        .unwrap_or_else(|_| {
                            exit(0)
                        });
                }
                Top::Enum(schema_enum) => {
                    builder.add_enum(schema_enum.to_owned(), schema.0.clone())
                        .unwrap_or_else(|_| {
                            exit(0)
                        });
                }
                Top::CompositeType(comp) => {
                    // builder.add_datasource(comp);
                }
                Top::Model(model) => {
                    // builder.add_datasource(model);
                }
                Top::Type(field) => {
                    // Not handled
                }
            }
        });
    }

    // builder.print();

    builder.render()
}
