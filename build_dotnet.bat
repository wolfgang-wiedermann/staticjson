python tools/generate_code.py  ./templates/dotnet_template.ct c > ./src/generator/dotnet.rs
python tools/generate_code.py  ./templates/knockout_template.ct c > ./src/generator/knockout.rs
cargo build
.\target\debug\staticjson.exe .\target\debug\test_studenten.sjs -t jquery --debug
.\target\debug\staticjson.exe .\target\debug\test_studenten.sjs -t knockout --debug
.\target\debug\staticjson.exe ./target\debug\test_studenten.sjs -t dotnet --debug
