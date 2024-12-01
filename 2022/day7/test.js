import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const commands = data.split('\n');
let total = 0;
for (const command of commands) {
  const commandElems = command.split(' ');
  if (commandElems[0] !== 'dir' && commandElems[0] !== '$') {
    total += parseInt(commandElems[0]);
  }
}

console.log(total);