var ffi = require('ffi');

var murmur = ffi.Library('lib/target/release/libmurmur.so', {
  murmur: ['int', ['string', 'int']]
});

module.exports = function(input, seed) {
  input = Buffer.isBuffer(input) ? input.toString('hex') : input;
  return murmur.murmur(input, seed || 0x1234567);
};
