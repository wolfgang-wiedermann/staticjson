all: template
	cargo build

template: 
	./tools/generate_code.py ./templates/htmldoc_template.ct c > ./src/generator/htmldoc.rs
	./tools/generate_code.py ./templates/jsoninc_template.ct c > ./src/generator/jsoninc.rs
	./tools/generate_code.py ./templates/swift_template.ct c > ./src/generator/swift.rs

test: all
	./target/debug/staticjson ./target/debug/test_studenten.sjs -t swift --debug
	
copy: test 
	cp output/* ../../Desktop/develop/ParserSample/ParserSample/
