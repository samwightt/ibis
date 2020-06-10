# Ibis

A documentation package manager, viewer, and search engine. Use Ibis to download documentation packages for offline use, view them in any web browser, and search through them using lightning fast search.

You can download any documentation package that has a `ibis.json` file in it. `ibis.json` files allow nested documentation structures to be laid out independently of their file system layouts. They are optimized for quick, no-transform-needed serving from web servers. They are strictly validated against a JSON schema, found in the `schema.json` file in this repo.

Ibis is written in Rust. The JSON Schema is generated using Typescript. It is currently in a pre-alpha stage and has a large part of the needed features unimplemented.

## Feature list

- [x] Verify `ibis.json` files.
- [ ] Standard format for documentation packages.
  - [x] Support for pages.
  - [ ] Support for multiple versions.
  - [ ] Support for specifying programming language.
  - [ ] Support for multiple languages in each package.
- [ ] Install and manage documentation packages
  - [ ] Download and manage documentation packages.
  - [ ] Remove and re-install documentation packages.
  - [ ] Install specific versions of documentation packages.
  - [ ] List all documentation packages installed.
- [ ] View downloaded packages in a web browser.
- [ ] Search through all content in downloaded packages via the CLI.
- [ ] Search through all content in downloaded packages via the web interface.
- [ ] Quickly switch between documentation packages using keyboard shortcuts.
- [ ] Search all documentation packages on the repo.
- [ ] Format for a repository of all documentation packages for NPM-like install behavior.
- [ ] Default language support.

## To Use

Because Ibis is pre-alpha software, there are no pre-built binaries of it available for download. In order to build Ibis locally, you'll need to install the latest version of [RustUp](https://rustup.rs/) in order to get things working. 

Once you have RustUp installed, run the following commands to build and run the CLI:

```
git clone https://github.com/samwightt/ibis/
cd ibis/cli
cargo run
```

You should see Ibis' help panel be outputted to the terminal.

## To Develop

Downloading and building Ibis for development is nearly identical to running it just for use:

```
git clone https://github.com/samwightt/ibis/
cd ibis/cli
cargo run
```

## Editing The Schema

To edit the `schema.json` file, you'll need the latest version of [NodeJS](https://nodejs.org/en/) installed. Once you have Node and NPM installed, run the following commands (assuming you have the directory cloned).

```
cd ibis/schema
npm install --save
npx ts-node compile.ts
```

This will compile the `schema.json` file from the `schema/schema.ts` file.

**Do not edit the contents of the schema.json file directly.** They are automatically generated by a compiler, and editing the contents directly can result in inconsistent contents.

To edit the actual structure and types of the schema, you'll need to know [Typescript](https://www.typescriptlang.org/). The `RootType` interface in the `schema.ts` file is the interface that the JSON Schema compiler uses for the root JSON type. All TS-Doc comments made above interfaces, types, enums, etc. are included in the `schema.json` as descriptions.

The `compile.ts` file is responsible for compiling the `schema.json` file. It *must* be run with `ts-node`, which should be automatically installed when you run `npm install --save`. The command for compiling is:

```
npx ts-node compile.ts
# Or, if ts-node is installed globally
ts-node compile.ts
```