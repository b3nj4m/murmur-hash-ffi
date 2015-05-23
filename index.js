var ffi = require('ffi');

var murmur = ffi.Library('lib/target/release/libmurmur.so', {
  murmur: ['int', ['string']]
});

module.exports = murmur.murmur;
