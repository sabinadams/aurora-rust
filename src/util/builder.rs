use datamodel;
use std::error::Error;
use std::vec;

pub struct Builder {
    datamodel: datamodel::dml::Datamodel,
    config: datamodel::Configuration,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            datamodel: datamodel::dml::Datamodel::new(),
            config: datamodel::Configuration {
                datasources: vec![],
                generators: vec![],
            },
        }
    }

    /// Registers a datasource in the builder. Throws an error if the existing datasource _(if any)_ doesn't match the provided one.
    /// <br/> **FUTURE**: This should merge datasources and throw more specific errors about non-matching options along with problem file(s).
    pub fn add_datasource(&mut self, source: datamodel::Datasource) -> Result<(), Box<dyn Error>> {
        if self.config.datasources.len() > 0 {
            // If this source is different than the existing one, throw an error.
            let old = datamodel::mcf::render_sources_to_json(&[source]);
            let new = datamodel::mcf::render_sources_to_json(&self.config.datasources);
            if old.ne(&new) {
                eprintln!("Please ensure all datasources are defined with the same configuration.");
                Err("error".into())
            } else {
                Ok(())
            }
        } else {
            self.config.datasources.push(source);
            Ok(())
        }
    }

    /// Registers a generator. It takes in a generator. If another generator exists already with exactly the same config, it does nothing.
    /// Otherwise it will add it.
    pub fn add_generator(&mut self, generator: datamodel::Generator) {
        let generators: Vec<String> = self
            .config
            .generators
            .clone()
            .into_iter()
            .map(|gen| {
                datamodel::mcf::generators_to_json(&[gen])
                    .as_str()
                    .to_owned()
            })
            .collect();

        if !generators.contains(
            &datamodel::mcf::generators_to_json(&[generator.clone()])
                .as_str()
                .to_owned(),
        ) {
            self.config.generators.push(generator);
        }
    }

    /// Registers an enum. If an existing enum's name matches the one provided,
    /// the values of the enum will be merged, resulting in one enum with both values.
    pub fn add_enum(&mut self, schema_enum: datamodel::dml::Enum) {
        // Get the position of a matching enum
        let enum_index = self
            .datamodel
            .enums
            .iter()
            .position(|enm| enm.name == schema_enum.name);

        if enum_index.is_none() {
            self.datamodel.enums.push(schema_enum);
        } else {
            for value in schema_enum.values {
                if self.datamodel.enums[enum_index.unwrap()]
                    .find_value(&value.name)
                    .is_none()
                {
                    self.datamodel.enums[enum_index.unwrap()].add_value(value);
                }
            }
        }
    }

    pub fn add_composite_type(&mut self, composite: datamodel::dml::CompositeType) {
        // Get the position of a matching enum
        let composite_index = self
            .datamodel
            .composite_types
            .iter()
            .position(|cmp| cmp.name == composite.name);

        if composite_index.is_none() {
            self.datamodel.composite_types.push(composite);
        } else {
            let existing_fields = &self.datamodel.composite_types[composite_index.unwrap()];
            let new_fields = &composite.fields;
        }
    }

    fn consolidate_attributes() {}

    pub fn print(&self) {
        println!("{:?}", self.config.datasources);
    }

    pub fn render(self) -> (datamodel::Configuration, datamodel::dml::Datamodel) {
        println!(
            "{}",
            datamodel::render_datamodel_and_config_to_string(&self.datamodel, &self.config)
        );
        (self.config, self.datamodel)
    }
}
