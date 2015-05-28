all: template
	cargo build

template: 
	./tools/generate_code ./templates/htmldoc_template.ct c > ./src/generator/htmldoc.rs
	./tools/generate_code ./templates/jsoninc_template.ct c > ./src/generator/jsoninc.rs
	./tools/generate_code ./templates/swift_template.ct c > ./src/generator/swift.rs

test:
	./target/debug/staticjson ./target/debug/test_studenten.sjs -t swift
