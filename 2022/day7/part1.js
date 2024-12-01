import fs from 'fs';

let fileSystem = {
  name: '/',
  directories: [],
  from: null,
  files: [],
}

const changeDir = (newDir) => {
  if (newDir === '..') {  
    current = current.from;
  }
  current = current.directories.find((directory) => directory === newDir);
};

const data = String(fs.readFileSync('./input.txt'));
const commands = data.split('\n');
let current = fileSystem;
for (const command of commands) {
  const commandElems = command.split(' ');
  console.log(commandElems);
  console.log(fileSystem);
  if (commandElems[1] === 'cd') {
    changeDir(commandElems[2]);
  }
  if (commandElems[0] === 'dir') {
    if (current.directories.length === 0) {
      current.directories = [];
    }
    current.directories.push(
      {
        name: commandElems[1],
        directories: ['..'],
        from: current,
        files: [],
      }
    );
  } else if (commandElems[0] !== '$') {
    current.files.push(parseInt(commandElems[1]));
  }
}

let totalDirs = 0;

current = fileSystem;
countDirs(current);
console.log(totalDirs);



const countDirs = (current) => {
  for (const dir of current.directories) {
    countDirs(dir);
  }
  if (dirSize(current) <= 100000) {
    totalDirs++;
  }
};

const dirSize = (current) => {
  const totalSize = 
    current.directories.reduce(
      (accumulator, currentValue) => dirSize(accumulator) + currentValue, 0
    ) + current.files.reduce(
      (accumulator, currentValue) => accumulator += currentValue, 0
    );
  return totalSize;
};
