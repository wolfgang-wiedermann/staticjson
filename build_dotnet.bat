python tools/generate_code.py  ./templates/dotnet_template.ct c > ./src/generator/dotnet.rs
python tools/generate_code.py  ./templates/dotnet_types_template.ct c > ./src/generator/dotnet_types.rs
python tools/generate_code.py  ./templates/dotnet_interfaces_template.ct c > ./src/generator/dotnet_interfaces.rs
python tools/generate_code.py  ./templates/knockout_template.ct c > ./src/generator/knockout.rs
python tools/generate_code.py  ./templates/jquery_template.ct c > ./src/generator/jquery.rs

cargo build

.\target\debug\staticjson.exe .\target\debug\test_studenten.sjs -t jquery --debug
.\target\debug\staticjson.exe .\target\debug\test_studenten.sjs -t knockout
.\target\debug\staticjson.exe ./target\debug\test_studenten.sjs -t dotnet 

.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t jquery -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Scripts\kdv\
.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t knockout -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Scripts\kdv\
.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t dotnet_typ -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.PlandatenLib\
.\target\debug\staticjson.exe C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\Meta\schnittstelle.sjs -t dotnet_ifa -o C:\Users\wiw39784\Documents\git\KDV.Plandaten\KDV.Plandaten\

