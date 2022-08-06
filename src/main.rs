mod syntax;
use syntax::data_type;
use syntax::control_flow;
use syntax::error_handler;
use syntax::memory_management;
use syntax::generic_types;
mod examples;
use examples::car_factory::car_factory_1;
use examples::car_factory::car_factory_2;
use examples::car_factory::car_factory_3;
use examples::car_factory::car_factory_4;
use examples::car_factory::car_factory_5;
use examples::person_name;
use examples::read_file;
use examples::copy_and_return;
use examples::generic_container;
use examples::iterator;
use examples::authentication_mod;
use examples::visibility;
use examples::mod_example;

fn main() {
	data_type::main();
	control_flow::main();
	car_factory_1::main();
	car_factory_2::main();
	car_factory_3::main();
	car_factory_4::main();
	car_factory_5::main();

	error_handler::main();
	person_name::main();
	read_file::main();

	memory_management::main();
	copy_and_return::main();

	generic_types::main();
	generic_container::main();
	iterator::main();
	
	authentication_mod::main();
	visibility::main();
	mod_example::main();
}

#[test]
fn data_type() {
	data_type::main();
}

#[test]
fn control_flow() {
	control_flow::main();
}

#[test]
fn car_factory_1() {
	car_factory_1::main();
}

#[test]
fn car_factory_2() {
	car_factory_2::main();
}

#[test]
fn car_factory_3() {
	car_factory_3::main();
}

#[test]
fn car_factory_4() {
	car_factory_4::main();
}

#[test]
fn car_factory_5() {
	car_factory_5::main();
}

#[test]
fn error_handler() {
	error_handler::main();
}

#[test]
fn person_name() {
	person_name::main();
}

#[test]
fn read_file() {
	read_file::main();
}

#[test]
fn memory_management() {
	memory_management::main();
}

#[test]
fn copy_and_return() {
	copy_and_return::main();
}

#[test]
fn generic_types() {
	generic_types::main();
}

#[test]
fn generic_container() {
	generic_container::main();
}

#[test]
fn iterator() {
	iterator::main();
}

#[test]
fn authentication_mod() {
	authentication_mod::main();
}

#[test]
fn visibility() {
	visibility::main();
}

#[test]
fn mod_example() {
	mod_example::main();
}
