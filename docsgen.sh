#!/bin/bash

function insert_tags () {
  awk '{sub(/<\/head>/,"<link rel=\"stylesheet\" href=\"/criterion.css\">\n</head>"); print}' $1 > docs/tmp.html && mv docs/tmp.html $1
}
export -f insert_tags

mkdir -p ./docs
rm -rf ./docs/*
# Following line commented because it will significantly increase build time.
# We should clean manually.
# cargo clean
rm -rf ./target/criterion
rm -rf ./target/doc
RUSTDOCFLAGS='--extend-css assets/extended.css --html-in-header assets/head.html --html-after-content assets/after-content.html' cargo doc --no-deps
cargo bench --bench benchmarks -- --plotting-backend plotters --color always
cp -r target/doc/* ./docs/
cp -r target/criterion ./docs/
find docs/criterion/ -name "*.html" -type f -print0 | xargs -0 -I {} bash -c 'insert_tags "{}"'
cp docs/theme.css docs/static.files/theme.css
cp assets/criterion.css docs/criterion.css
cp -r assets/images ./docs/