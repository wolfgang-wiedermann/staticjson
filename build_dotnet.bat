python tools/generate_code.py  ./templates/dotnet_template.ct c > ./src/generator/dotnet.rs
python tools/generate_code.py  ./templates/knockout_template.ct c > ./src/generator/knockout.rs
cargo build
.\target\debug\staticjson.exe .\target\debug\test_studenten.sjs -t jquery --debug
.\target\debug\staticjson.exe .\target\debug\test_studenten.sjs -t knockout --debug
.\target\debug\staticjson.exe ./target\debug\test_studenten.sjs -t dotnet --debug


.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t jquery -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Scripts\kdv\
.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t knockout -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Scripts\kdv\
.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t dotnet -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.PlandatenLib\

