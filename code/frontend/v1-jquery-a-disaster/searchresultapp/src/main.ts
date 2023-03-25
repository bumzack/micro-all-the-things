/// <reference path ="../node_modules/@types/jquery/jquery.d.ts"/>
/// <amd-dependency>


class SearchResultComp extends HTMLElement {

    static get observedAttributes() {
        return ['urlPath'];
    }

    // @ts-ignore
    attributeChangedCallback(property, oldValue, newValue) {
        console.log(`attr change  property ${property}  from ${oldValue} to ${newValue}`);
        if (oldValue === newValue) return;
        // @ts-ignore
        this[property] = newValue;
    }

    constructor() {
        super();
        console.log("constructor SearchResultComp");
    }

    connectedCallback() {
        console.log("search result comp connectedCallback");
        this.innerHTML += this.createSearchResultApp();

        this.searchButtonClickHandler();
        this.categoryClickHandler();
        'doCategoryArticles'
    }

    private categoryClickHandler() {
        let that = this;
        let doc = document.getElementById("search-result-app");

        doc.addEventListener(doCategoryClickEvent, (e: CustomEvent) => {
            const categoryCode = e.detail.categoryCode as string;
            console.log('loading articles for category code ${}')
            // https://stackoverflow.com/questions/511947/jquerys-ajax-is-causing-a-full-page-refresh-in-firefox
            e.preventDefault();

            const searchRequest: SearchResult = {
                code: categoryCode,
                start: 0,
                pageSize: 50
            };

            let url = "http://localhost:8200/solr/search/article/code"
            console.log(`sending POST to url ${url} with request data ${JSON.stringify(searchRequest, null, 4)}`);

            $.ajax({
                url: url,
                type: "POST",
                data: JSON.stringify(searchRequest),
                contentType: 'application/json',
                success: function (data, textStatus, jqXHR) {
                    console.log(`success - textStatus   ${JSON.stringify(textStatus, null, 4)}`);
                    console.log(`success - found articles ${JSON.stringify(data, null, 4)}`);
                    if (data !== null && data.length > 0) {
                        const articles = data as Array<Article>;
                        that.displayArticles(articles);
                        $(".searchresult-text").text(articles.length + " Artikel gefunden");
                    } else {
                        $(".searchresult-text").text(`In Kategorie mit Code '${categoryCode}"' hat keine Artikel gefunden.`);
                    }
                    const cnt = data.length;
                    console.log(`success - found articles ${cnt}`);
                },
                error: function (jqXHR, textStatus, errorThrown) {
                    console.log(`error -   ${textStatus}`);
                    console.log(`error -   ${errorThrown}`);
                    console.log(textStatus);
                    console.log(errorThrown);
                }
            });
        });
    }

    private searchButtonClickHandler() {
        let that = this;
        let doc = document.getElementById("search-result-app");

        $(idSearchResultApp).on(doSearchClickEvent, function (a) {
            console.log("a " + JSON.stringify(a, null, 4));
        })
        doc.addEventListener(doSearchClickEvent, (e: CustomEvent) => {

            // https://stackoverflow.com/questions/511947/jquerys-ajax-is-causing-a-full-page-refresh-in-firefox
            e.preventDefault();

            // console.log(e.detail.searchText);

            //  ¯\_(ツ)_/¯
            //  https://github.com/microsoft/TypeScript/issues/28357

            let searchText = e.detail.searchText as string;
            console.log(`searchText: ${searchText}`);

            const searchRequest: SearchResult = {
                text: searchText,
                start: 0,
                pageSize: 50
            };

            let url = "http://localhost:8300/solr/search/article/text"
            console.log(`sending POST to url ${url} with request data ${JSON.stringify(searchRequest, null, 4)}`);

            $.ajax({
                url: url,
                type: "POST",
                data: JSON.stringify(searchRequest),
                contentType: 'application/json',
                success: function (data, textStatus, jqXHR) {
                    console.log(`success - textStatus   ${JSON.stringify(textStatus, null, 4)}`);
                    console.log(`success - found articles ${JSON.stringify(data, null, 4)}`);
                    if (data !== null && data.length > 0) {
                        const articles = data as Array<Article>;
                        that.displayArticles(articles);
                        $(".searchresult-text").text(articles.length + " Artikel gefunden");
                    } else {
                        $(".searchresult-text").text(`Suche nach '${searchText}"' hat keine Artikel gefunden.`);
                    }
                    const cnt = data.length;
                    console.log(`success - found articles ${cnt}`);
                },
                error: function (jqXHR, textStatus, errorThrown) {
                    console.log(`error -   ${textStatus}`);
                    console.log(`error -   ${errorThrown}`);
                    console.log(textStatus);
                    console.log(errorThrown);
                }
            });
        });
    }

    displayArticles = (articles: Array<Article>): void => {
        $("#searchResultArticleList").empty();
        articles.forEach(article => {
            console.log(`adding article ${article.product.code}`);

            const htmlArticle = $(this.getArticleView());
            $(htmlArticle).find(".article-name").text(article.product.articleName);
            if (article.product.articleDescription != null) {
                $(htmlArticle).find(".article-description").text(article.product.articleDescription.slice(0, 200));
            }
            $(htmlArticle).find(".article-code").text("Artikelnummer:" + article.product.code);

            if (article.image !== null) {
                console.log(`setting URL to  ${article.image.url}`);
                $(htmlArticle).find(".article-image").attr("src", article.image.url);
            }

            $("#searchResultArticleList").append(htmlArticle);
        });
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
        // @ts-ignore
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

window.customElements.define('search-result-comp', SearchResultComp);