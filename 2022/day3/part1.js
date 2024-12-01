import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const sacks = data.split("\n");
let total = 0;
for (const sack of sacks) {
  const length = Math.floor(sack.length / 2);
  const compartmentOne = sack.substr(0, length);
  const compartmentTwo = sack.substr(length);
  for (let i = 0; i < compartmentOne.length; i++) {
    if (compartmentTwo.includes(compartmentOne.charAt(i))) {
      if (compartmentOne.charAt(i) === compartmentOne.charAt(i).toUpperCase()) {
        total += compartmentOne.charCodeAt(i) - 'A'.charCodeAt(0) + 27;
      } else {
        total += compartmentOne.charCodeAt(i) - 'a'.charCodeAt(0) + 1;
      }
      break;
    }
  }
}
console.log(total);
