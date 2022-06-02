use std::vec;
use datamodel;

pub struct Builder {
    datamodel: datamodel::dml::Datamodel,
    config: datamodel::Configuration
}

impl Builder {
    pub fn new () -> Builder {
        Builder {
            datamodel: datamodel::dml::Datamodel::new(),
            config: datamodel::Configuration {
                datasources: vec![],
                generators: vec![],
            }
        }
    }
    pub fn add_datasource(&mut self, source: datamodel::Datasource) {
        self.config.datasources.push(source);
    }

    pub fn print(&self) {
        println!("{:?}", self.config.datasources);
    }

    pub fn render(self) -> (datamodel::Configuration, datamodel::dml::Datamodel) {
        (
            self.config,
            self.datamodel
        )
    }
}