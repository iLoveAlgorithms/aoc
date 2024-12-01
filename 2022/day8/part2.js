import fs from 'fs';

const data = String(fs.readFileSync('./input.txt'));
const rowStr = data.split('\n');
const rows = [];
const cols = [];
for (let i = 0; i < rowStr[0].length; i++) {
  cols[i] = [];
}
for (let i = 0; i < rowStr.length; i++) {
  rows[i] = [...rowStr[i]];
  for (let j = 0; j < rowStr[i].length; j++) {
    rows[i][j] = parseInt(rows[i][j]);
    cols[j].push(parseInt(rows[i][j]));
  }
}

let mostScenic = 0;

for (let i = 1; i < rowStr.length - 1; i++) {
  for (let j = 1; j < rowStr[i].length - 1; j++) {
    if (scenicScore(i, j) > mostScenic) {
      mostScenic = scenicScore(i, j);
    }
  }
}

console.log(mostScenic);

function scenicScore(a, b) {
  let score = 1;
  let current = 1;
  for (let j = b + 1; j < cols.length - 1; j++) {
    if (rows[a][b] <= rows[a][j]) {
      break;
    }
    current++;
  }

  score *= current;
  current = 1;
  for (let j = b - 1; j > 0; j--) {
    if (rows[a][b] <= rows[a][j]) {
      break;
    }
    current++;
  }

  score *= current;
  current = 1;
  for (let i = a + 1; i < rows.length - 1; i++) {
    if (rows[a][b] <= rows[i][b]) {
      break;
    }
    current++;
  }

  score *= current;
  current = 1;
  for (let i = a - 1; i > 0; i--) {
    if (rows[a][b] <= rows[i][b]) {
      break;
    }
    current++;
  }

  score *= current;

  return score;
}
