const encoding = require("./Encoding.js");
const decoding = require("./Decoding.js");

const encoded = encoding.edgeEncode("Hello World!");
const decoded = decoding.edgeDecode(encoded);

console.log("Encoded:\t" + encoded);
console.log("Decoded:\t" + decoded);
