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