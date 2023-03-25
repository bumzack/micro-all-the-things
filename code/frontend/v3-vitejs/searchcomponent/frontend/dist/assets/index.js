(function polyfill() {
  const relList = document.createElement("link").relList;
  if (relList && relList.supports && relList.supports("modulepreload")) {
    return;
  }
  for (const link of document.querySelectorAll('link[rel="modulepreload"]')) {
    processPreload(link);
  }
  new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      if (mutation.type !== "childList") {
        continue;
      }
      for (const node of mutation.addedNodes) {
        if (node.tagName === "LINK" && node.rel === "modulepreload")
          processPreload(node);
      }
    }
  }).observe(document, { childList: true, subtree: true });
  function getFetchOpts(link) {
    const fetchOpts = {};
    if (link.integrity)
      fetchOpts.integrity = link.integrity;
    if (link.referrerPolicy)
      fetchOpts.referrerPolicy = link.referrerPolicy;
    if (link.crossOrigin === "use-credentials")
      fetchOpts.credentials = "include";
    else if (link.crossOrigin === "anonymous")
      fetchOpts.credentials = "omit";
    else
      fetchOpts.credentials = "same-origin";
    return fetchOpts;
  }
  function processPreload(link) {
    if (link.ep)
      return;
    link.ep = true;
    const fetchOpts = getFetchOpts(link);
    fetch(link.href, fetchOpts);
  }
})();
const DO_SEARCH_EVENT = "doSearch";
const ID_SEARCH_RESULT_APP = "search-result-component";
window.$ = window.jQuery = jQuery;
class SearchComp extends HTMLElement {
  constructor() {
    super();
    console.log("search component constructor called");
  }
  createSearchApp() {
    return `<input id="searchtext"  type="search" class="form-control" placeholder="Search..." aria-label="Search"/>`;
  }
  connectedCallback() {
    console.log("search component callback");
    this.innerHTML += this.createSearchApp();
    const txt = document.getElementById("searchtext");
    jQuery("#searchtext").keydown((event) => {
      if (event.key === "Enter") {
        if (txt.value != null) {
          console.log("sending custom event with  search text: '", txt.value, "'");
          const doc = document.getElementById(ID_SEARCH_RESULT_APP);
          if (doc !== null) {
            doc.dispatchEvent(
              new CustomEvent(DO_SEARCH_EVENT, {
                detail: {
                  "searchText": txt.value
                }
              })
            );
          }
        }
      }
    });
  }
}
window.customElements.define("search-component", SearchComp);
