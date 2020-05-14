const pathname = window.location.pathname;

if (pathname.endsWith('html')) {
  const gitalk = new Gitalk({
    clientID: '49aeab30501e804c71b0',
    clientSecret: '6d3c8320ac0c5c461afc2c01994833f05056b0de',
    repo: 'sf-zhou.github.io',
    owner: 'SF-Zhou',
    admin: ['SF-Zhou'],
    id: pathname,
  });

  gitalk.render('gitalk-container');
}

if (pathname === "/" || pathname === "/index.html") {
  window.onhashchange = function () {
    const mark = window.location.hash.substr(2);
    const cards = document.getElementsByClassName("card");
    for (card of cards) {
      var found = false;
      const tags = card.getElementsByClassName("tag");
      for (tag of tags) {
        const a = tag.children[0];
        if (a.innerText === mark) {
          tag.classList.add("selected");
          a.href = "#";
          found = true;
        } else {
          tag.classList.remove("selected");
          a.href = "#/" + a.innerText;
        }
      }
      card.hidden = mark.length > 0 && !found;
    }
  }

  window.onhashchange();
}

// Google Analytics
(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){ (i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o), m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m) })(window,document,'script','https://www.google-analytics.com/analytics.js','ga'); ga('create', 'UA-61723712-2', 'auto'); ga('send', 'pageview');
