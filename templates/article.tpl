<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title_string }}</title>
  </head>
  <body>
    <div id="app">
      {{ article }}
    </div>
    <script>
      window.content = {
        title: {{ title }},
        date: {{ date }},
        author: {{ author }},
        tags: {{ tags }}
      }
    </script>
    <script src="/dist/build.js"></script>
  </body>
</html>
