var ffi = require('ffi');
var fs = require('fs');
var Path = require('path');

var path = 'lib/target/release/libmurmur';
var murmur = ffi.Library(Path.join(__dirname, path), {
  cMurmur: ['uint32', ['string', 'uint32']]
});

module.exports = function(input, seed) {
  input = Buffer.isBuffer(input) ? input.toString('hex') : input;
  return murmur.cMurmur(input, seed || 0x1234567);
};
