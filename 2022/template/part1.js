import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const somethings = data.split('\n');
for (const something of somethings) {
  console.log(something);
}