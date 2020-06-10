import * as fs from "fs";
import { resolve } from "path";

import * as TJS from "typescript-json-schema";

const settings: TJS.PartialArgs = {
  required: true,
};

const compilerOptions: TJS.CompilerOptions = {
  strictNullChecks: true,
};

const program = TJS.getProgramFromFiles(
  [resolve("./schema.ts")],
  compilerOptions
);

const schema = TJS.generateSchema(program, "RootType", settings);

fs.writeFile("../schema.json", JSON.stringify(schema, null, 4), (err) => {
  if (err) console.log("There was an error creating the file.");
  console.log("Generated schema!");
});

fs.writeFile("../schema.min.json", JSON.stringify(schema), (err) => {
  if (err) console.log("There was an error generating the minified file.");
  console.log("Generated minified schema!");
});
