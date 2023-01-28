#!/bin/bash


mkdir -p ./docs
rm -rf ./docs/*
RUSTDOCFLAGS='--extend-css assets/extended.css --html-in-header assets/head.html --html-after-content assets/after-content.html' cargo doc
cp -r target/doc/* ./docs/