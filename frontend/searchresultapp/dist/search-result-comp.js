class SearchResultComp extends HTMLElement {
    static get observedAttributes() {
        return ['urlPath'];
    }
    attributeChangedCallback(property, oldValue, newValue) {
        console.log(`attr change  property ${property}  from ${oldValue} to ${newValue}`);
        if (oldValue === newValue)
            return;
        this[property] = newValue;
    }
    constructor() {
        super();
        this.displayArticles = (articles) => {
            $("#searchResultArticleList").empty();
            articles.forEach(article => {
                console.log(`adding article ${article.article.code}`);
                const htmlArticle = $(this.getArticleView());
                $(htmlArticle).find(".article-name").text(article.article.articleName);
                $(htmlArticle).find(".article-description").text(article.article.articleDescription.slice(0, 200));
                $(htmlArticle).find(".article-code").text("Artikelnummer:" + article.article.code);
                if (article.image !== null) {
                    console.log(`setting URL to  ${article.image.url}`);
                    $(htmlArticle).find(".article-image").attr("src", article.image.url);
                }
                $("#searchResultArticleList").append(htmlArticle);
            });
        };
        console.log("constructor SearchResultComp");
    }
    connectedCallback() {
        console.log("blalalalal");
        this.innerHTML += this.createSearchResultApp();
        let doc = document.getElementById("searchapp");
        let that = this;
        doc.addEventListener("doSearch", (e) => {
            console.log(e.detail.searchText);
            let searchText = e.detail.searchText;
            console.log(`searchText: ${searchText}`);
            const searchRequest = {
                code: null,
                text: searchText,
                start: 0,
                pageSize: 50
            };
            let url = "http://localhost:8300/solr/search/article/text";
            console.log(`sending POST to url ${url} with request data ${JSON.stringify(searchRequest)}`);
            $.ajax({
                url: url,
                type: "POST",
                data: JSON.stringify(searchRequest),
                contentType: 'application/json',
                success: function (data, textStatus, jqXHR) {
                    console.log(`success - textStatus   ${JSON.stringify(textStatus, null, 4)}`);
                    console.log(`success - found articles ${JSON.stringify(data, null, 4)}`);
                    if (data !== null && data.length > 0) {
                        const articles = data;
                        that.displayArticles(articles);
                        $(".searchresult-text").text(articles.length + " Artikel gefunden");
                    }
                    else {
                        $(".searchresult-text").text(`Suche nach '${searchText}"' hat keine Artikel gefunden.`);
                    }
                    const cnt = data.length;
                    console.log(`success - found articles ${cnt}`);
                },
                error: function (jqXHR, textStatus, errorThrown) {
                    console.log(`error -   ${textStatus}`);
                    console.log(`error -   ${errorThrown}`);
                    console.error(textStatus);
                    console.error(errorThrown);
                }
            });
        });
    }
    getArticleView() {
        return ` 
            <div class="col">
                <div class="card">
                    <div class="row g-0">
                        <div class="col-md-4">
                            <svg class="bd-placeholder-img" width="100%" height="250" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: Image" preserveAspectRatio="xMidYMid slice" focusable="false">
                            <title>Placeholder</title>
                                <rect width="100%" height="100%">
                                    <div width="100%" height="100%">
                                        <img class="card-img-top article-image" style="height: 225px; width: 100%; display: block;">                            
                                    </div>
                                </rect>
                            </svg>

                        </div>
                        <div class="col-md-8">
                            <div class="card-body">
                                <h5 class="card-title article-name" ></h5>
                                <p class="card-text description"></p>
                                <p class="card-text article-code"><small class="text-muted"> </small></p>
                            </div>
                        </div>
                    </div>
                </div>>
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
window.customElements.define('search-result-comp', SearchResultComp);
export {};
//# sourceMappingURL=search-result-comp.js.map