
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct Team {
    name: String,
    pts: u16
}

impl Team {
	pub fn new(name: String, pts: u16)-> Team{
		Team{name: name, pts: pts}
	}
}

impl ToJson for Team {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("pts".to_string(), self.pts.to_json());
        m.to_json()
    }
}
