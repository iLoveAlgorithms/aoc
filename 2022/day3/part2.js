import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const sacks = data.split("\n");

const letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
let total = 0;
let group = [];
let count = 0;
for (const sack of sacks) {
  if (count < 3) {
    group.push(sack);
    count++;
  } 
  
  if (count === 3) {
    for (let i = 0; i < letters.length; i++) {
      const letter = letters.charAt(i);
      if (group[0].includes(letter) && group[1].includes(letter) && group[2].includes(letter)) {
        total += letters.indexOf(letter) + 1;
      }
    }
    count = 0;
    group = [];
  }
}
console.log(total);
