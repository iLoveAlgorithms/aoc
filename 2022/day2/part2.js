import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const rounds = data.split("\n");
let totalPoints = 0;
for (const round of rounds) {
  if (round.charAt(2) === 'Y') {
    totalPoints += 3;
  } else if (round.charAt(2) === 'Z') {
    totalPoints += 6;
  }

  if (round.charAt(2) === 'X') {
    const choosePoints = round.charCodeAt(0) === 65 ?  3 : round.charCodeAt(0) - 65;
    totalPoints += choosePoints;
  } else if (round.charAt(2) === 'Y') {
    const choosePoints = round.charCodeAt(0) - 64;
    totalPoints += choosePoints;
  } else if (round.charAt(2) === 'Z') {
    const choosePoints = round.charCodeAt(0) === 67 ?  1 : round.charCodeAt(0) - 63;
    totalPoints += choosePoints;
  }
}
console.log(totalPoints);