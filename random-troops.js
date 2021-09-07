const crypto = require("crypto");
const ffi = require("ffi-napi");
const ref = require("ref-napi");

const arrayType = require("ref-array-di")(ref);

// define the "int[]" type
const randTroopsArray = arrayType("uint32");
const randTroopsArrayPtr = ref.refType(randTroopsArray);

// Rust library path
const randGenlibPath = "target/release/librand_gen.so";
const lib = ffi.Library(randGenlibPath, {
  rand_generate: ["size_t", ["string", "uint32", "uint32", randTroopsArrayPtr]],
  rand_free: ["void", [randTroopsArray, "size_t"]],
});

function randomTroops(player, armyTypes, armySize) {
  const reqStr = `${player}${armyTypes.join("")}${armySize}`;
  const sha256Hasher = crypto.createHmac("sha256", reqStr);
  const hashedReq = sha256Hasher.update(reqStr).digest("base64");
  const bufPtr = ref.alloc(randTroopsArrayPtr);
  const arraySize = lib.rand_generate(
    hashedReq,
    armyTypes.length,
    armySize,
    bufPtr
  );
  const armies = randTroopsArray(bufPtr.deref());
  armies.length = arraySize;

  try {
    let result = [];
    for (let i = 0; i < armies.length; i++) {
      result.push({ name: armyTypes[i], troops: armies[i] });
    }
    return result;
  } finally {
    lib.rand_free(armies, arraySize);
  }
}

module.exports = randomTroops;
