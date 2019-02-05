const fs = require('mz/fs');
const path = require('path');

async function mkdir(dir_path) {
  if (await fs.exists(dir_path)) {
    if ((await fs.stat(dir_path)).isFile()) {
      throw `${dir_path} is a FILE`;
    }
  } else {
    mkdir(path.dirname(dir_path));
    await fs.mkdir(dir_path);
  }
}

module.exports = mkdir;
