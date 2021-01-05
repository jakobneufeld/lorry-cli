use lorry;
fn main() {
    lorry::Program::new()
    .author("Jakob Neufeld")
    .name("lorry-cli")
    .version("0.1.4")
    .dependencie("indicatif", "0.15.0")
    .gen();
}
