#!/usr/local/bin/node

var http = require('http');

var httpProxy = require('http-proxy');

var options = {};
// Create a standalone HTTPS proxy server
//httpProxy.createServer(8000, 'localhost', options).listen(8001);

//
// Create an instance of HttpProxy to use with another HTTPS server
//
var proxy = new httpProxy.HttpProxy({
    target: {
        host: 'api.github.com',
        port: 443,
        https: true
    },
    enable: {
        xforward: false // enables X-Forwarded-For
    },
    changeOrigin: true
});

http.createServer(function(req, res) {
    proxy.proxyRequest(req, res);
}).listen(8002);

console.log('Requests to http://localhost:8002 will be proxied to https://api.github.com');