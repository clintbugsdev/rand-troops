const each = require("jest-each").default;
const randomTroops = require("./random-troops");

var len = 100;
var args = new Array(len);
for (let i = 0; i < len; i++) {
  args.push({
    i: i,
    a: "Clinton",
    b: [
      "a",
      "b",
      "c",
      "d",
      "e",
      "f",
      "g",
      "h",
      "i",
      "j",
      "k",
      "l",
      "m",
      "n",
      "o",
      "p",
      "q",
      "r",
      "s",
      "t",
      "u",
      "v",
      "w",
      "x",
      "y",
      "z",
    ],
    c: 1000,
    expected: 26,
  });
}
each(args).test("test $i x ", ({ i, a, b, c, expected }) => {
  expect(randomTroops(a, b, c).length).toBe(expected);
});
