## murmur-hash-ffi

A Rust implementation of the non-cryptographic murmur hash, with a node interface provided via [FFI](https://npmjs.org/package/ffi).

```javascript
var murmur = require('murmur-hash-ffi');

console.log(murmur('cheese'));
console.log(murmur('beans', 42));
```
