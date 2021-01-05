use lorry;
fn main() {
    lorry::Program::new()
    .author("Jakob Neufeld")
    .name("lorry-cli")
    .version("0.1.4")
    .readme("Readme.md")
    .description("A cli for lorry")
    .dependencie("indicatif", "0.15.0")
    .gen();
}
