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

    pub fn print(&self) {
        println!("{:?}", self.config.datasources);
    }

    pub fn render(self) -> (datamodel::Configuration, datamodel::dml::Datamodel) {
        println!("{}", datamodel::render_datamodel_and_config_to_string(&self.datamodel, &self.config));
        (self.config, self.datamodel)
    }
}
