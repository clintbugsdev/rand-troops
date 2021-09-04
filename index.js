const ffi = require("ffi-napi");
const ref = require("ref-napi");
const arrayType = require("ref-array-di")(ref);

// define the "int[]" type
const randTroopsArray = arrayType("uint32");
const randTroopsArrayPtr = ref.refType(randTroopsArray);

// Rust library path
const randGenlibPath = "target/release/librand_gen.so";
const lib = ffi.Library(randGenlibPath, {
  rand_generate: ["size_t", ["uint32", "uint32", randTroopsArrayPtr]],
  rand_free: ["void", [randTroopsArray, "size_t"]],
});

function randGenerate(armyTypes, armySize) {
  const bufPtr = ref.alloc(randTroopsArrayPtr);
  const arraySize = lib.rand_generate(armyTypes.length, armySize, bufPtr);
  const armies = randTroopsArray(bufPtr.deref());
  armies.length = arraySize;

  try {
    let str = "";
    for (let i = 0; i < armies.length; i++) {
      str += armies[i] + ", ";
    }
    return str;
  } finally {
    lib.rand_free(armies, arraySize);
  }
}

console.log(randGenerate(["Spearmen", "Swordsmen", "Archer"], 100));
