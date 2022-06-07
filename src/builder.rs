use datamodel::{self, ast::Attribute};
use std::{error::Error, path::PathBuf};
pub struct Builder {
    ast: datamodel::ast::SchemaAst
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            ast: datamodel::ast::SchemaAst::empty()
        }
    }

   fn compare_items(&self, a: datamodel::ast::Top, b: datamodel::ast::Top) -> bool {
        let mut ast_a = datamodel::ast::SchemaAst::empty();
        let mut ast_b = datamodel::ast::SchemaAst::empty();

        ast_a.tops.push(a);
        ast_b.tops.push(b);
        
        datamodel::render_schema_ast_to_string(&ast_a).eq(
            &datamodel::render_schema_ast_to_string(&ast_b)
        )
    }

    /// Registers a datasource in the builder. Throws an error if the existing datasource _(if any)_ doesn't match the provided one.
    /// <br/> **FUTURE**: This should merge datasources and throw more specific errors about non-matching options along with problem file(s).
    pub fn add_datasource(&mut self, source: datamodel::ast::SourceConfig, path: PathBuf) ->  Result<(), Box<dyn Error>> {
        if self.ast.sources().count() == 0 {
            self.ast.tops.push(
                datamodel::ast::Top::Source(source)
            );
            Ok(())
        } else {
            let old = self.ast.sources().next().unwrap().to_owned();
            if self.compare_items(
                datamodel::ast::Top::Source(source),
                datamodel::ast::Top::Source(old)
            ){
                Ok(())
            } else {
                eprintln!("Please ensure all datasources are defined with the same configuration.");
                Err("error".into())
            }
        }
    }
    /// Registers a generator. It takes in a generator. If another generator exists already with exactly the same config, it does nothing.
    /// Otherwise it will add it.
    pub fn add_generator(&mut self, generator: datamodel::ast::GeneratorConfig, path: PathBuf ) -> Result<(), Box<dyn Error>> {
        if self.ast.generators().count() == 0 {
            self.ast.tops.push(
                datamodel::ast::Top::Generator(generator)
            );
            Ok(())
        } else {
            let existing = self.ast.generators().find(|gen| {
                gen.name == generator.name
            });
            if existing.is_some() {
                if self.compare_items(
                    datamodel::ast::Top::Generator(generator),
                    datamodel::ast::Top::Generator(existing.unwrap().to_owned())
                ) {
                    Ok(())
                } else {
                    eprintln!("You cannot have more than one generator with the same name unless they are configured exactly the same.");
                    Err("error".into())
                }
            } else {
                self.ast.tops.push(
                    datamodel::ast::Top::Generator(generator)
                );
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
