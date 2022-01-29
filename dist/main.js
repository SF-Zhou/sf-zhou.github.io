const pathname = window.location.pathname;

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
