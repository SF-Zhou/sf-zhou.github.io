<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title_string }}</title>
  </head>
  <body>
    <div id="app"></div>
    <script>
      window.content = {
        title: "{{ title }}",
        date: "{{ date }}",
        author: "{{ author }}",
        article: "{{ article }}"
      }
    </script>
    <script src="/dist/build.js"></script>
  </body>
</html>
