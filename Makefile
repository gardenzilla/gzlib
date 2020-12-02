.PHONY: test, dev, doc, publish

build:
	cargo build

publish:
	# git add .
	# git commit -m ".."
	# git push
	cargo check
	cargo publish --no-verify

test:
	cargo test

doc:
	docker run --rm\
		-v $(shell pwd)/docs:/out\
		-v $(shell pwd)/src/proto:/protos\
		pseudomuto/protoc-gen-doc
	docker run --rm\
		-v $(shell pwd)/docs:/out\
		-v $(shell pwd)/src/proto:/protos\
		pseudomuto/protoc-gen-doc\
		--doc_opt=markdown,index.md