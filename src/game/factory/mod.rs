pub trait Factory {
	fn create_controller();
	fn create_input();
	fn create_view();
}

struct AbstractFactory {
	legacy_factory: &'static dyn Factory,
	extended_factory: &'static dyn Factory
}