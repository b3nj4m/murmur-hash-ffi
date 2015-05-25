var ffi = require('ffi');

var murmur = ffi.Library('lib/target/release/libmurmur', {
  cMurmur: ['uint32', ['string', 'uint32']]
});

module.exports = function(input, seed) {
  input = Buffer.isBuffer(input) ? input.toString('hex') : input;
  return murmur.cMurmur(input, seed || 0x1234567);
};
