<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{{ title_string }}}</title>
  </head>
  <body>
    <div id="app">
      <div id="original_article" hidden>
        {{{ article }}}
      </div>
    </div>
    <script>
      window.content = {
        index: {{{ index }}},
        title: {{{ title }}},
        date: {{{ date }}},
        author: {{{ author }}},
        tags: {{{ tags }}}
      }
    </script>
    <script src="/dist/build.js"></script>
    <script>
    (function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){ (i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o), m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m) })(window,document,'script','https://www.google-analytics.com/analytics.js','ga'); ga('create', 'UA-61723712-2', 'auto'); ga('send', 'pageview');
    </script>
  </body>
</html>
