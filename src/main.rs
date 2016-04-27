#![cfg_attr(all(feature="serde_type"), feature(custom_derive, plugin))]
#![cfg_attr(all(feature="serde_type"), plugin(serde_macros))]

extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate env_logger;
extern crate handlebars_iron as hbs;
#[cfg(not(feature = "serde_type"))]
extern crate rustc_serialize;
#[cfg(feature = "serde_type")]
extern crate serde;
#[cfg(feature = "serde_type")]
extern crate serde_json;
#[macro_use]
extern crate maplit;


extern crate serde_json;
extern crate serde;
use std::error::Error;

use iron::prelude::*;
use iron::{status};
use iron::headers::*;
use router::Router;
use router::NoRoute;
use hbs::{Template, HandlebarsEngine, DirectorySource};

use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

//use serde_json::value::*;


use mount::Mount;
use staticfile::Static;

use std::path::Path;

use std::env;

use iron::{ AfterMiddleware};


struct MenuItem {
	name: String,
	selected: bool
}

impl MenuItem {
	pub fn new(name: String, selected: bool)-> MenuItem{
		MenuItem{name: name, selected: selected}
	}
}
impl ToJson for MenuItem {
	fn to_json(&self) -> Json {
		let mut m: BTreeMap<String, Json> = BTreeMap::new();
		m.insert("name".to_string(), self.name.to_json());
		m.insert("selected".to_string(), self.selected.to_json());
		m.to_json()
	}
}



fn menu_items(current_item: &str) -> Vec<MenuItem> {
	vec![ MenuItem::new("search".to_string(), "search" == current_item), MenuItem::new("compare".to_string(), "compare" == current_item), MenuItem::new("stats".to_string(), "stats" == current_item)]
}


fn base_page_data(data: &mut BTreeMap<String, Json>, page_name: &str) {
	data.insert("nav_menu_elements".to_string(), menu_items(page_name).to_json());
	data.insert("server_adress".to_string(), get_adress().to_json());
	data.insert("server_port".to_string(), get_port().to_json());

}

struct Custom404;
impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
	    let mut resp = Response::new();
		resp.headers.set(Location(format!("http://{}:{}/error/404", get_adress(), get_port()).to_owned()));
		resp.set_mut(status::MovedPermanently);




        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(resp)
        } else {
            Err(err)
        }
    }
}

fn error_404(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with(status::NotFound);
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "404");

    data.insert("parent".to_string(), "page_template".to_json());
    data.insert("page-name".to_string(), "404".to_json());
    data.insert("title".to_string(), "404 Error: page not found!".to_json());

    resp.set_mut(Template::new("error/404", data));
	resp.set_mut(status::NotFound);

    Ok(resp)
}

fn index(_: &mut Request) -> IronResult<Response> {

    let mut resp = Response::new();
	resp.headers.set(Location(format!("http://{}:{}/page/search", get_adress(), get_port()).to_owned()));
	resp.set_mut(status::MovedPermanently);

	Ok(resp)
}
fn search(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "search");

    data.insert("parent".to_string(), "page_template".to_json());
    data.insert("page-name".to_string(), "search".to_json());
    data.insert("title".to_string(), "Search!".to_json());
    resp.set_mut(Template::new("page/search", data)).set_mut(status::Ok);

    Ok(resp)
}

fn compare(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "compare");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page-name".to_string(), "compare".to_json());
    data.insert("title".to_string(), "Compare!".to_json());

    resp.set_mut(Template::new("page/compare", data)).set_mut(status::Ok);

    Ok(resp)
}

fn stats(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "stats");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page-name".to_string(), "stats".to_json());
    data.insert("title".to_string(), "Stats!".to_json());

    resp.set_mut(Template::new("page/stats", data)).set_mut(status::Ok);

    Ok(resp)
}

fn get_adress() -> String{

	match env::args().nth(1){
		Some(value)=>{
			value
		},
		None=>"localhost".to_string()
	}
}
fn get_port() -> String{

	match env::args().nth(2){
		Some(value)=>{
			value
		},
		None=>"8080".to_string()
	}
}


fn main() {

    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();

    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }


    let mut router = Router::new();
    router.get("/", index);
    router.get("/error/404", error_404);
    router.get("/page/search", search);
    router.get("/page/compare", compare);
    router.get("/page/stats", stats);

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);
    chain.link_after(hbse);

	let mut mount = Mount::new();


	mount.mount("/", chain);
	mount.mount("style", Static::new(Path::new("style/")));
	mount.mount("fonts", Static::new(Path::new("fonts/")));
	mount.mount("code", Static::new(Path::new("code/")));
	mount.mount("img", Static::new(Path::new("img/")));

    println!("Server running at http://{}:{}", get_adress(), get_port());
    Iron::new(mount).http(format!("{}:{}", get_adress(), get_port()).as_str()).unwrap();
}
