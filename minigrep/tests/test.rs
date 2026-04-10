use minigrep::Config;

#[test]
fn fail() {
    let args = ["./minigrep"];

    Config::build(args.iter());
}
