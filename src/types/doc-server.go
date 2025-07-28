package types

const DocsHtml = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>Document</title>
    <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
    <meta name="description" content="Description" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, minimum-scale=1.0"
    />
    <link
      rel="stylesheet"
      href="//cdn.jsdelivr.net/npm/docsify@4/lib/themes/vue.css"
    />
  </head>
  <body>
    <div id="app"></div>
    <script>
      window.$docsify = {
        name: '',
        repo: '',
        loadSidebar: true,
      };
    </script>
    <!-- Docsify v4 -->
    <script src="//cdn.jsdelivr.net/npm/docsify@4"></script>
  </body>
</html>`

const DocsPackageJson = `{
  "name": "markdown-server",
  "version": "1.0.0",
  "description": "A simple HTTP server that serves Markdown files as HTML",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js",
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [
    "markdown",
    "server",
    "http",
    "html",
    "static-site"
  ],
  "author": "",
  "dependencies": {
    "docsify": "^4.13.1",
    "marked": "^9.1.6"
  },
  "devDependencies": {
    "nodemon": "^3.0.2"
  },
  "engines": {
    "node": ">=14.0.0"
  }
}`
