# Docubus

A documentation package manager, viewer, and search engine. Use Docubus to download documentation packages for offline use, view them in any web browser, and search through them using lightning fast search.

You can download any documentation package that has a `docubus.json` file in it. `docubus.json` files allow nested documentation structures to be laid out independently of their file system layouts. They are optimized for quick, no-transform-needed serving from web servers. They are strictly validated against a JSON schema, found in the `schema.json` file in this repo.

Docubus is written in Rust. The JSON Schema is generated using Typescript.