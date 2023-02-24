#!/bin/bash

while [[ $# -gt 0 ]]; do
  case $1 in
    --doc-only)
      DOC_ONLY='1'
      shift
      ;;
  esac
done


function insert_tags () {
  awk '{sub(/<\/head>/,"<link rel=\"stylesheet\" href=\"/criterion.css\">\n</head>"); print}' $1 > docs/tmp.html && mv docs/tmp.html $1
}
export -f insert_tags

mkdir -p ./docs
rm -rf ./docs/*
# With following line commented because it will significantly increase 
# build time. We should clean manually.
# cargo clean
rm -rf ./target/criterion
rm -rf ./target/doc

# Cargo doc
RUSTDOCFLAGS='--extend-css assets/extended.css --html-in-header assets/head.html --html-after-content assets/after-content.html' cargo doc --no-deps
cp -r target/doc/* ./docs/
cp docs/theme.css docs/static.files/theme.css
cp -r assets/images ./docs/

if [[ $DOC_ONLY != '1' ]]; then 
  # Cargo benchmark
  cargo bench --bench benchmarks -- --plotting-backend plotters --color always
  cp -r target/criterion ./docs/
  find docs/criterion/ -name "*.html" -type f -print0 | xargs -0 -I {} bash -c 'insert_tags "{}"'
  cp assets/criterion.css docs/criterion.css
fi 
