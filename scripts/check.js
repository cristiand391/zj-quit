const util = require('node:util');
const exec = util.promisify(require('node:child_process').exec);

exports.preCommit = async () => {
  await exec('cargo check')
};
