all: template
	cargo build

template: 
	./tools/generate_code.py ./templates/htmldoc_template.ct c > ./src/generator/htmldoc.rs
	./tools/generate_code.py ./templates/jsoninc_template.ct c > ./src/generator/jsoninc.rs
	./tools/generate_code.py ./templates/jaxrs_template.ct c > ./src/generator/jaxrs.rs
	./tools/generate_code.py ./templates/java_client_template.ct c > ./src/generator/java_client.rs
	./tools/generate_code.py ./templates/jquery_template.ct c > ./src/generator/jquery.rs

test: all
	./target/debug/staticjson ./target/debug/test_studenten.sjs -t jquery --debug
	./target/debug/staticjson ./target/debug/test_studenten.sjs -t jaxrs --debug
	
copy: test 
	cp output/* ../../Desktop/develop/ParserSample/ParserSample/
