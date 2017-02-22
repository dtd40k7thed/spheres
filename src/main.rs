
#[macro_use]
extern crate qmlrs;

#[macro_use]
extern crate ejdb;

use ejdb::Database;

fn main() {

	let db = Database::open("./db").unwrap();

	let coll = db.collection("collection").unwrap();

	let mut d = bson! {
		"name" => "Foo Bar",
		"count" => 10
	};
	let id = coll.save(&d).unwrap();

	coll.load(id).unwrap();

	let mut engine = qmlrs::Engine::new();

	engine.load_local_file("main.qml");

	engine.exec();

}
