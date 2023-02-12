#!/bin/bash


mkdir -p ./docs
rm -rf ./docs/*
RUSTDOCFLAGS='--extend-css assets/extended.css --html-in-header assets/head.html --html-after-content assets/after-content.html' cargo doc --no-deps
cp -r target/doc/* ./docs/
cp docs/theme.css docs/static.files/theme.css
cp -r assets/images ./docs/