use datamodel::{self};
use std::{error::Error, path::PathBuf};
use serde_json::{json, Value};
pub struct Builder {
    ast: datamodel::ast::SchemaAst,
    datasources: Vec<(String, String)>,
    generators: Vec<(String, String)>,
    models: Vec<(String, String)>,
    composite_types: Vec<(String, String)>,
    enums: Vec<(String, String)>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            ast: datamodel::ast::SchemaAst::empty(),
            datasources: Vec::new(),
            generators: Vec::new(),
            models: Vec::new(),
            composite_types: Vec::new(),
            enums: Vec::new(),
        }
    }


    /// Takes two Top items, adds them to a schema, renders the schema to a string, and determines whether or not the two are the same.
    /// Returns true or false.
    fn compare_items(&self, a: datamodel::ast::Top, b: datamodel::ast::Top) -> bool {
        let mut ast_a = datamodel::ast::SchemaAst::empty();
        let mut ast_b = datamodel::ast::SchemaAst::empty();

        ast_a.tops.push(a);
        ast_b.tops.push(b);
        
        datamodel::render_schema_ast_to_string(&ast_a).eq(
            &datamodel::render_schema_ast_to_string(&ast_b)
        )
    }

    fn record_path(&mut self, item_type: &str, name: String, path: PathBuf) {
        let value = (name, path.display().to_string());
        match item_type {
            "datasource" => self.datasources.push(value),
            "generator" => self.generators.push(value),
            "model" => self.models.push(value),
            "composite_type" => self.composite_types.push(value),
            "enum" => self.enums.push(value),
            _ => println!("Skipping {}", item_type)
        }
    }

    /// Registers a datasource in the builder. Throws an error if the existing datasource _(if any)_ doesn't match the provided one.
    /// <br/> **FUTURE**: This should merge datasources and throw more specific errors about non-matching options along with problem file(s).
    pub fn add_datasource(&mut self, source: &datamodel::ast::SourceConfig, path: PathBuf) ->  Result<(), Box<dyn Error>> {
        if self.ast.sources().count() == 0 {
            self.ast.tops.push(
                datamodel::ast::Top::Source(source.clone())
            );
            self.record_path("datasource", source.name.name.to_owned(), path);
            Ok(())
        } else {
            let old = self.ast.sources().next().unwrap().to_owned();
            if self.compare_items(
                datamodel::ast::Top::Source(source.to_owned()),
                datamodel::ast::Top::Source(old)
            ){
                Ok(())
            } else {
                eprintln!("Please ensure all datasources are defined with the same configuration.");
                let source_name = source.name.name.to_owned();
                let source_path = self.datasources.iter().find(|spath| spath.0 == source_name).unwrap();
                eprintln!("Datasource in [{}] was configured differently than the datasource in [{}]", source_path.1, path.display());
                Err("error".into())
            }
        }
    }
    /// Registers a generator. It takes in a generator. If another generator exists already with exactly the same config, it does nothing.
    /// Otherwise it will add it.
    pub fn add_generator(&mut self, generator: &datamodel::ast::GeneratorConfig, path: PathBuf ) -> Result<(), Box<dyn Error>> {
        if self.ast.generators().count() == 0 {
            self.ast.tops.push(
                datamodel::ast::Top::Generator(generator.clone())
            );
            self.record_path("generator", generator.name.name.to_owned(), path);
            Ok(())
        } else {
            let existing = self.ast.generators().find(|gen| {
                gen.name == generator.name
            });
            if existing.is_some() {
                if self.compare_items(
                    datamodel::ast::Top::Generator(generator.to_owned()),
                    datamodel::ast::Top::Generator(existing.unwrap().to_owned())
                ) {
                    Ok(())
                } else {
                    eprintln!("You cannot have more than one generator with the same name unless they are configured exactly the same.");
                    let gen_name = generator.name.name.to_owned();
                    let gen_path = self.generators.iter().find(|gen| gen.0 == gen_name).unwrap();
                    eprintln!("Generator in [{}] was configured differently than the generator in [{}]", gen_path.1, path.display());
                    Err("error".into())
                }
            } else {
                self.ast.tops.push(
                    datamodel::ast::Top::Generator(generator.clone())
                );
                self.record_path("generator", generator.name.name.to_owned(), path);
                Ok(())
            }
        }
    }

    /// Registers an enum. If an existing enum's name matches the one provided,
    /// the values of the enum will be merged, resulting in one enum with both values.
    // pub fn add_enum(&mut self, schema_enum: datamodel::dml::Enum) {
    //     // Get the position of a matching enum
    //     let enum_index = self
    //         .datamodel
    //         .enums
    //         .iter()
    //         .position(|enm| enm.name == schema_enum.name);

    //     if enum_index.is_none() {
    //         self.datamodel.enums.push(schema_enum);
    //     } else {
    //         for value in schema_enum.values {
    //             if self.datamodel.enums[enum_index.unwrap()]
    //                 .find_value(&value.name)
    //                 .is_none()
    //             {
    //                 self.datamodel.enums[enum_index.unwrap()].add_value(value);
    //             }
    //         }
    //     }
    // }
    pub fn add_enum(&mut self, mut schema_enum: datamodel::ast::Enum, path: PathBuf ) -> Result<(), Box<dyn Error>> {
        let enums: Vec<datamodel::ast::Top> = self.ast.tops.clone().into_iter().filter(|top| {
            top.get_type() == "enum"
        }).collect();
        
        let position = enums.iter().position(|top| {
            top.name() == schema_enum.name.name
        });

        if position.is_some() {
            let mut test = enums[position.unwrap()].as_enum().unwrap().to_owned();
            test.values.append(&mut schema_enum.values);
            Ok(())
        } else {
            self.ast.tops.push(
                datamodel::ast::Top::Enum(schema_enum.clone())
            );
            Ok(())
        }
    }

    // pub fn add_composite_type(&mut self, composite: datamodel::dml::CompositeType) {
    //     // Get the position of a matching enum
    //     let composite_index = self
    //         .datamodel
    //         .composite_types
    //         .iter()
    //         .position(|cmp| cmp.name == composite.name);

    //     if composite_index.is_none() {
    //         self.datamodel.composite_types.push(composite);
    //     } else {
    //         let existing_fields = &self.datamodel.composite_types[composite_index.unwrap()];
    //         let new_fields = &composite.fields;
    //     }
    // }

    fn consolidate_attributes() {}

    pub fn print(&self) {
        println!("{:?}", self.ast);
    }

    pub fn render(self) {
        println!(
            "{}",
            datamodel::render_schema_ast_to_string(&self.ast)
        );
    }
}
