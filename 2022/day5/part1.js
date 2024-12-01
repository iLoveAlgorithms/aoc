import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const mvmts = data.split('\n');
let towers = [
  [],
  ['Q', 'F', 'L', 'S', 'R'],
  ['T', 'P', 'G', 'Q', 'Z', 'N'],
  ['B', 'Q', 'M', 'S'],
  ['Q', 'B', 'C', 'H', 'J', 'Z', 'G', 'T'],
  ['S', 'F', 'N', 'B', 'M', 'H', 'P'],
  ['G', 'V', 'L', 'S', 'N', 'Q', 'C', 'P'],
  ['F', 'C', 'W'],
  ['M', 'P', 'V', 'W', 'Z', 'G', 'H', 'Q'],
  ['R', 'N', 'C', 'L', 'D', 'Z', 'G'],
];

for (let i = 0; i < 10; i++) {
  mvmts.shift();
}
for (const mvmt of mvmts) {
  const actions = mvmt.split(' ');
  const numMvmts = parseInt(actions[1]);
  const fromStack = parseInt(actions[3]);
  const toStack = parseInt(actions[5]);
  for (let i = 0; i < numMvmts; i++) {
    const moveCrate = towers[fromStack].shift();
    towers[toStack].unshift(moveCrate);
  }
}

console.log(towers);