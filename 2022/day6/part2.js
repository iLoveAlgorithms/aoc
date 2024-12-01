import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
let currentData = [];
let currDataPositions = [];
for (let i = 0; i < data.length; i++) {
  for (let j = i; currentData.length < 14; j++) {
    if (currentData.includes(data.charAt(j))) {
      break;
    }
    else {
      currentData.push(data.charAt(j));
      currDataPositions.push(j);
    }
  }

  if (currentData.length === 14) {
    break;
  }
  currentData = [];
  currDataPositions = [];
}
console.log(currDataPositions);