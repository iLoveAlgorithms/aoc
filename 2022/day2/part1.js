import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const rounds = data.split("\n");
let totalPoints = 0;
for (const round of rounds) {
  if ((round.charCodeAt(0) + 23) === round.charCodeAt(2)) {
    totalPoints += 3;
  } else if (
    ((round.charCodeAt(0) + 24) === round.charCodeAt(2)) ||
    ((round.charAt(0) === 'C') && (round.charAt(2) === 'X'))
  ) {
    totalPoints += 6;
  }

  const choosePoints = round.charCodeAt(2) - 87;
  totalPoints += choosePoints;
}
console.log(totalPoints);