
#[macro_use]
extern crate qmlrs;

fn main() {

	let mut engine = qmlrs::Engine::new();

	engine.load_local_file("main.qml");

	engine.exec();

}
