python tools/generate_code.py  ./templates/dotnet_template.ct c > ./src/generator/dotnet.rs
cargo build
./target/debug/staticjson ./target/debug/test_studenten.sjs -t jquery --debug
./target/debug/staticjson ./target/debug/test_studenten.sjs -t dotnet --debug
