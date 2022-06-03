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
        
        if self.config.datasources.len() > 0 {
            let old = datamodel::mcf::render_sources_to_json(&[source]);
            let new = datamodel::mcf::render_sources_to_json(&self.config.datasources);
            if old != new {
                self.config.datasources[0].active_connector = source.active_connector;
            }
        }
        // self.config.datasources.push(source);
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