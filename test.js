var murmur = require('./index');

var expect = require('chai').expect;

describe('murmur', function() {
  it('should work', function() {
    expect(murmur).to.be.a('function');
    expect(murmur('cheese', 42)).to.equal(2799699609);
    expect(murmur('beans', 42)).to.equal(3425272181);
  });
});
