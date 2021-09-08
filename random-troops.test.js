const randomTroops = require("./random-troops");
// describe("randomTroops", () => {
//   beforeEach(() => {
//     randomTroops = require("./random-troops");
//   });
//   //   it("size: 10", () => {
//   //     expect(
//   //       randomTroops("Clinton", ["Spearmen", "Swordsmen", "Archer"], 10).length
//   //     ).toEqual(3);
//   //   });

//   //   it("works too", () => {
//   //     expect(
//   //       randomTroops("Clinton", ["Spearmen", "Swordsmen", "Archer"], 100).length
//   //     ).toBe(3);
//   //   });
// });

test("size: 1,000", () => {
  expect(
    randomTroops(
      "Clinton",
      ["Spearmen", "Swordsmen", "Archer", "Mace", "Healer", "Catapult"],
      1000
    ).length
  ).toBe(6);
});

test("size: 10,000", () => {
  expect(
    randomTroops(
      "Clinton",
      ["Spearmen", "Swordsmen", "Archer", "Mace", "Healer", "Catapult"],
      10000
    ).length
  ).toBe(6);
});

test("size: 1000,000", () => {
  expect(
    randomTroops(
      "Clinton",
      ["Spearmen", "Swordsmen", "Archer", "Mace", "Healer", "Catapult"],
      100000
    ).length
  ).toBe(6);
});

// test("size: 10000", () => {
//   expect(
//     randomTroops("Clinton", ["Spearmen", "Swordsmen", "Archer"], 10000).length
//   ).toBe(3);
// });
