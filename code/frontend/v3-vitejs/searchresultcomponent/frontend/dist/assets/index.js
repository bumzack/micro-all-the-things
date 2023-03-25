var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => {
  __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
  return value;
};
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
class SearchResultComp extends HTMLElement {
  constructor() {
    super();
    __publicField(this, "displayArticles", (articles) => {
      $("#searchResultArticleList").empty();
      articles.forEach((article) => {
        console.log(`adding article ${article.product.code}`);
      });
    });
    console.log("constructor SearchResultComponent");
  }
  static get observedAttributes() {
    return ["urlPath"];
  }
  // @ts-ignore
  attributeChangedCallback(property, oldValue, newValue) {
    console.log(`attr change  property ${property}  from ${oldValue} to ${newValue}`);
    if (oldValue === newValue)
      return;
    this[property] = newValue;
  }
  connectedCallback() {
    console.log("search result component connectedCallback");
    this.innerHTML += this.createSearchResultApp();
    this.searchButtonClickHandler();
  }
  searchButtonClickHandler() {
    let that = this;
    let doc = document.getElementById(ID_SEARCH_RESULT_APP);
    if (doc !== null) {
      doc.addEventListener(DO_SEARCH_EVENT, (e) => {
        e.preventDefault();
        let st = e.detail;
        let searchText = st.searchText;
        console.log(`searchText: ${searchText}`);
        const searchRequest = {
          text: searchText,
          start: 0,
          pageSize: 50
        };
        let url = "http://localhost:8300/solr/search/article/text";
        console.log(`sending POST to url ${url} with request data ${JSON.stringify(searchRequest, null, 4)}`);
        $.ajax({
          url,
          type: "POST",
          data: JSON.stringify(searchRequest),
          contentType: "application/json",
          success: function(data, textStatus) {
            console.log(`success - textStatus   ${JSON.stringify(textStatus, null, 4)}`);
            console.log(`success - found articles ${JSON.stringify(data, null, 4)}`);
            if (data !== null && data.length > 0) {
              const articles = data;
              that.displayArticles(articles);
              $(".searchresult-text").text(articles.length + " Artikel gefunden");
            } else {
              $(".searchresult-text").text(`Suche nach '${searchText}"' hat keine Artikel gefunden.`);
            }
            const cnt = data.length;
            console.log(`success - found articles ${cnt}`);
          },
          error: function(_jqXHR, textStatus, errorThrown) {
            console.log(`error -   ${textStatus}`);
            console.log(`error -   ${errorThrown}`);
            console.log(textStatus);
            console.log(errorThrown);
          }
        });
      });
    }
  }
  getArticleView() {
    return ` 
            <div class="col">
                <div class="card">
                    <div class="row g-0">
                        <div class="col-md-4">
                            <div class="bd-placeholder-img" width="100%" height="250"  role="img" aria-label="Placeholder: Image"  focusable="false">
                                <img class="card-img-top article-image" style="height: 225px; width: 100%; display: block;"/>                               
                            </div>
                        </div>
                        <div class="col-md-8">
                            <div class="card-body">
                                <h5 class="card-title article-name" ></h5>
                                <p class="card-text description"></p>
                                <p class="card-text article-code"><small class="text-muted"> </small></p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
           `;
  }
  createSearchResultApp() {
    return `
            <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
                <h1 class="h2 searchresult-text">Dashboard</h1> 
            </div>
            <div class="album py-5 bg-light">
                <div id="searchResult"  class="container">
                    <div id="searchResultArticleList" class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
                   
                    </div>
                </div>
           </div>`;
  }
}
window.customElements.define("search-result-component", SearchResultComp);
