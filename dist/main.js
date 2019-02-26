const pathname = window.location.pathname;

if (pathname.endsWith('html') && pathname.indexOf("/hidden/") === -1) {
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
