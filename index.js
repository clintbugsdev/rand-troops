const randomTroops = require("./random-troops");

let player = "Clinton";
let armyTypes = ["Spearmen", "Swordsmen", "Archer"];
let armySize = 4;
console.log(randomTroops(player, armyTypes, armySize));
