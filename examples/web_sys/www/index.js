var express = require('express');
var path = require('path');
var pathToBuild = process.argv[2] || '../../../target/wasm32-unknown-unknown/release';

express.static.mime.define({'application/wasm': ['wasm']});

var app = express();

app.use(express.static(path.resolve(pathToBuild), { index: false }));
app.use(express.static(path.resolve('./', __dirname)));

app.listen(8080);
