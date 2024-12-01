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

const visible = [];
for (let i = 0; i < rowStr.length; i++) {
  visible.push([]);
}

let count = 0;

for (let i = 0; i < rowStr.length; i++) {
  let current = -1;
  for (let j = 0; j < rowStr[i].length; j++) {
    if (rows[i][j] > current) {
      count++;
      current = rows[i][j];
      visible[i][j] = true;
    } else {
      visible[i][j] = false;
    }
  }

  current = -1;
  for (let j = rowStr[i].length - 1; j >= 0; j--) {
    if (rows[i][j] > current && !visible[i][j]) {
      count++;
      current = rows[i][j];
      visible[i][j] = true;
    }
  }
}

for (let i = 0; i < cols.length; i++) {
  let current = -1;
  for (let j = 0; j < cols[i].length; j++) {
    if (cols[i][j] > current) {
      current = cols[i][j];
        if (!visible[j][i]) {
          count++;
          visible[j][i] = true;
        }
    }
  }

  current = -1;
  for (let j = cols[i].length - 1; j >= 0; j--) {
    if (cols[i][j] > current) {
      current = cols[i][j];
        if (!visible[j][i]) {
          count++;
          visible[j][i] = true;
        }
    }
  }
}

console.log(count);
