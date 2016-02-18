python tools/generate_code.py  ./templates/jquery_template.ct c > ./src/generator/jquery.rs
cargo build
./target/debug/staticjson ./target/debug/test_studenten.sjs -t jquery --debug
./target/debug/staticjson ./target/debug/test_studenten.sjs -t jaxrs --debug
