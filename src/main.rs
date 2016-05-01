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
extern crate urlencoded;
extern crate serde_json;
extern crate serde;
extern crate uuid;

use std::error::Error;
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use router::Router;
use router::NoRoute;
use hbs::{DirectorySource, HandlebarsEngine, Template};
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use urlencoded::UrlEncodedQuery;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::env;
use iron::AfterMiddleware;
use serde::ser::Serializer;
use serde::ser::MapVisitor;
use serde::ser::impls::MapIteratorVisitor;
use std::process::Command;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use uuid::Uuid;

struct MenuItem {
	name: String,
	selected: bool,
}

impl MenuItem {
	pub fn new(name: String, selected: bool) -> MenuItem { MenuItem { name: name, selected: selected } }
}
impl ToJson for MenuItem {
	fn to_json(&self) -> Json {
		let mut m: BTreeMap<String, Json> = BTreeMap::new();
		m.insert("name".to_string(), self.name.to_json());
		m.insert("selected".to_string(), self.selected.to_json());
		m.to_json()
	}
}


fn menu_items(current_item: &str) -> Vec<MenuItem> { vec![MenuItem::new("search".to_string(), "search" == current_item), MenuItem::new("compare".to_string(), "compare" == current_item), MenuItem::new("stats".to_string(), "stats" == current_item), MenuItem::new("about".to_string(), "about" == current_item)] }


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


fn index(_: &mut Request) -> IronResult<Response> {

	let mut resp = Response::new();
	resp.headers.set(Location(format!("http://{}:{}/page/search", get_adress(), get_port()).to_owned()));
	resp.set_mut(status::MovedPermanently);

	Ok(resp)
}

fn error_404(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::with(status::NotFound);
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "404");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page_name".to_string(), "404".to_json());
	data.insert("title".to_string(), "404 Error: page not found!".to_json());

	resp.set_mut(Template::new("error/404", data));
	resp.set_mut(status::NotFound);

	Ok(resp)
}

fn search(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "search");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page_name".to_string(), "search".to_json());
	resp.set_mut(Template::new("page/search", data)).set_mut(status::Ok);

	Ok(resp)
}


fn about(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "about");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page_name".to_string(), "about".to_json());
	resp.set_mut(Template::new("page/about", data)).set_mut(status::Ok);

	Ok(resp)
}

fn compare(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "compare");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page_name".to_string(), "compare".to_json());
	resp.set_mut(Template::new("page/compare", data)).set_mut(status::Ok);

	Ok(resp)
}

fn stats(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	let mut data = BTreeMap::new();

	base_page_data(&mut data, "stats");

	data.insert("parent".to_string(), "page_template".to_json());
	data.insert("page_name".to_string(), "stats".to_json());
	resp.set_mut(Template::new("page/stats", data)).set_mut(status::Ok);

	Ok(resp)
}

struct SearchResult {
	document_id: String,
	name: String,
}

impl SearchResult {
	pub fn new(document_id: String, name: String) -> SearchResult { SearchResult { document_id: document_id, name: name } }
}

impl ToJson for SearchResult {
	fn to_json(&self) -> Json {
		let mut m: BTreeMap<String, Json> = BTreeMap::new();
		m.insert("documentId".to_string(), self.document_id.to_json());
		m.insert("name".to_string(), self.name.to_json());
		m.to_json()
	}
}


impl serde::Serialize for SearchResult {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		let mut m = BTreeMap::new();
		m.insert("documentId".to_string(), (&self.document_id).to_string());
		m.insert("name".to_string(), (&self.name).to_string());
		serializer.serialize_map(MapIteratorVisitor::new(m.iter(), Some(m.len())))
	}
}


fn search_by_query(query: &str, _result_limit: usize) -> Vec<SearchResult> {
	let query_file_name = format!("/tmp/{}", Uuid::new_v4());
	let mut query_file = File::create(&query_file_name).unwrap();

	query_file.write_all(format!("0:{}:criterion:txt\n", query).as_bytes()).unwrap();
    query_file.flush().unwrap();
    query_file.sync_data().unwrap();


	let search_command = Command::new("./sri_rs/target/release/querying")
 		                     .arg("sri_config.properties")
		                     .arg(&query_file_name)
							 .output()
		                     .unwrap();

 	std::fs::remove_file(&query_file_name).unwrap();

	let search_output = String::from_utf8_lossy(&search_command.stdout).into_owned();
	println!("{}", search_output);

	let search_err = String::from_utf8_lossy(&search_command.stderr).into_owned();
	println!("{}", search_err);

	let result_file = search_output.lines().last().unwrap();

	let mut search_results = Vec::new();

	match File::open(result_file) {
		Ok(file) => {
			let file = BufReader::new(&file);
			for (index, line) in file.lines().enumerate() {
				let line = line.unwrap();
				if !line.is_empty(){
					search_results.push(SearchResult::new(index.to_string(), line));
				}
			}
		},
		Err(e) => println!("{}", e)
	}

	search_results
}


fn subtitles_search(request: &mut Request) -> IronResult<Response> {
	let mut query = None;
	match request.get_ref::<UrlEncodedQuery>() {
		Ok(ref hashmap) => query = hashmap.get("query"),
		Err(ref e) => println!("{:?}", e),
	};

	let mut resp = Response::new();
	match query {
		Some(query) => {
			let search_results = search_by_query(&query[0], 10);

			let search_results_as_string = serde_json::to_string(&search_results).unwrap();
			resp.set_mut(search_results_as_string).set_mut(status::Ok);
		},
		None => {
			resp.set_mut("{error:\"No query provided\"}").set_mut(status::Ok);
			()
		},
	}

	Ok(resp)
}

fn document_id_to_filename(document_id: &str) -> String{
	document_id.to_string()
}

fn subtitles_get_file(request: &mut Request) -> IronResult<Response> {
	let mut document_id = None;
	match request.get_ref::<UrlEncodedQuery>() {
		Ok(ref hashmap) => document_id = hashmap.get("documentId"),
		Err(ref e) => println!("{:?}", e),
	};

	let mut resp = Response::new();
	match document_id {
		Some(document_id) => {
			let document_filename = document_id_to_filename(&document_id[0]);
			let document_filename_as_json = serde_json::to_string(&document_filename).unwrap();
			resp.set_mut(document_filename_as_json).set_mut(status::Ok);

		},
		None => {
			resp.set_mut("{error:\"No document id provided\"}").set_mut(status::Ok);
			()
		},
	}

	Ok(resp)
}

fn get_adress() -> String {

	match env::args().nth(1) {
		Some(value) => value,
		None => "localhost".to_string(),
	}
}

fn get_port() -> String {

	match env::args().nth(2) {
		Some(value) => value,
		None => "8080".to_string(),
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
	router.get("/page/about", about);
	router.get("/subtitles/search", subtitles_search);
	router.get("/subtitles/file", subtitles_get_file);

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
