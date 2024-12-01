import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const assignments = data.split("\n");
let count = 0;
for (const assignment of assignments) {
  const elves = assignment.split(",");
  let numberElfAss = [];
  for (let i = 0; i < 2; i++) {
    numberElfAss[i] = [];
    const elfAss = elves[i].split("-");
    for (let j = 0; j < 2; j++) {
      numberElfAss[i][j] = parseInt(elfAss[j]);
    }
  }

  if (
    (numberElfAss[0][0] >= numberElfAss[1][0]) && 
    (numberElfAss[1][1] >= numberElfAss[0][0])) {
    console.log(numberElfAss);
    count++;
  } else if (
    (numberElfAss[0][0] <= numberElfAss[1][0]) && 
    (numberElfAss[1][0] <= numberElfAss[0][1])) {
    console.log(numberElfAss);
    count++;
  }
}

console.log(count);