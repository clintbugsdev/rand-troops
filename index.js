const randomTroops = require("./random-troops");

let player = "Player 1";
let armyTypes = ["Spearmen", "Swordsmen", "Archer"];
let armySize = 10;
console.log(randomTroops(player, armyTypes, armySize));
