var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
define("common/dtos", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
});
define("common/const", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.tagCategoryDepth = exports.tagCategoryCode = exports.idSearchResultApp = exports.doCategoryClickEvent = exports.doSearchClickEvent = void 0;
    // Custom Event names
    exports.doSearchClickEvent = "doSearch";
    exports.doCategoryClickEvent = "doCategoryFilter";
    // App IDs
    exports.idSearchResultApp = "search-result-app";
    // custome element tags
    exports.tagCategoryCode = "categorycode";
    exports.tagCategoryDepth = "depth";
});
/// <reference path ="../node_modules/@types/jquery/jquery.d.ts"/>
/// <amd-dependency>
define("searchresultapp/src/main", ["require", "exports", "common/const"], function (require, exports, const_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    var SearchResultComp = /** @class */ (function (_super) {
        __extends(SearchResultComp, _super);
        function SearchResultComp() {
            var _this = _super.call(this) || this;
            _this.displayArticles = function (articles) {
                $("#searchResultArticleList").empty();
                articles.forEach(function (article) {
                    console.log("adding article ".concat(article.product.code));
                    var htmlArticle = $(_this.getArticleView());
                    $(htmlArticle).find(".article-name").text(article.product.articleName);
                    if (article.product.articleDescription != null) {
                        $(htmlArticle).find(".article-description").text(article.product.articleDescription.slice(0, 200));
                    }
                    $(htmlArticle).find(".article-code").text("Artikelnummer:" + article.product.code);
                    if (article.image !== null) {
                        console.log("setting URL to  ".concat(article.image.url));
                        $(htmlArticle).find(".article-image").attr("src", article.image.url);
                    }
                    $("#searchResultArticleList").append(htmlArticle);
                });
            };
            console.log("constructor SearchResultComp");
            return _this;
        }
        Object.defineProperty(SearchResultComp, "observedAttributes", {
            get: function () {
                return ['urlPath'];
            },
            enumerable: false,
            configurable: true
        });
        // @ts-ignore
        SearchResultComp.prototype.attributeChangedCallback = function (property, oldValue, newValue) {
            console.log("attr change  property ".concat(property, "  from ").concat(oldValue, " to ").concat(newValue));
            if (oldValue === newValue)
                return;
            // @ts-ignore
            this[property] = newValue;
        };
        SearchResultComp.prototype.connectedCallback = function () {
            console.log("search result comp connectedCallback");
            this.innerHTML += this.createSearchResultApp();
            this.searchButtonClickHandler();
            this.categoryClickHandler();
            'doCategoryArticles';
        };
        SearchResultComp.prototype.categoryClickHandler = function () {
            var that = this;
            var doc = document.getElementById("search-result-app");
            doc.addEventListener(const_1.doCategoryClickEvent, function (e) {
                var categoryCode = e.detail.categoryCode;
                console.log('loading articles for category code ${}');
                // https://stackoverflow.com/questions/511947/jquerys-ajax-is-causing-a-full-page-refresh-in-firefox
                e.preventDefault();
                var searchRequest = {
                    code: categoryCode,
                    start: 0,
                    pageSize: 50
                };
                var url = "http://localhost:8200/solr/search/article/code";
                console.log("sending POST to url ".concat(url, " with request data ").concat(JSON.stringify(searchRequest, null, 4)));
                $.ajax({
                    url: url,
                    type: "POST",
                    data: JSON.stringify(searchRequest),
                    contentType: 'application/json',
                    success: function (data, textStatus, jqXHR) {
                        console.log("success - textStatus   ".concat(JSON.stringify(textStatus, null, 4)));
                        console.log("success - found articles ".concat(JSON.stringify(data, null, 4)));
                        if (data !== null && data.length > 0) {
                            var articles = data;
                            that.displayArticles(articles);
                            $(".searchresult-text").text(articles.length + " Artikel gefunden");
                        }
                        else {
                            $(".searchresult-text").text("In Kategorie mit Code '".concat(categoryCode, "\"' hat keine Artikel gefunden."));
                        }
                        var cnt = data.length;
                        console.log("success - found articles ".concat(cnt));
                    },
                    error: function (jqXHR, textStatus, errorThrown) {
                        console.log("error -   ".concat(textStatus));
                        console.log("error -   ".concat(errorThrown));
                        console.log(textStatus);
                        console.log(errorThrown);
                    }
                });
            });
        };
        SearchResultComp.prototype.searchButtonClickHandler = function () {
            var that = this;
            var doc = document.getElementById("search-result-app");
            $(const_1.idSearchResultApp).on(const_1.doSearchClickEvent, function (a) {
                console.log("a " + JSON.stringify(a, null, 4));
            });
            doc.addEventListener(const_1.doSearchClickEvent, function (e) {
                // https://stackoverflow.com/questions/511947/jquerys-ajax-is-causing-a-full-page-refresh-in-firefox
                e.preventDefault();
                // console.log(e.detail.searchText);
                //  ¯\_(ツ)_/¯
                //  https://github.com/microsoft/TypeScript/issues/28357
                var searchText = e.detail.searchText;
                console.log("searchText: ".concat(searchText));
                var searchRequest = {
                    text: searchText,
                    start: 0,
                    pageSize: 50
                };
                var url = "http://localhost:8300/solr/search/article/text";
                console.log("sending POST to url ".concat(url, " with request data ").concat(JSON.stringify(searchRequest, null, 4)));
                $.ajax({
                    url: url,
                    type: "POST",
                    data: JSON.stringify(searchRequest),
                    contentType: 'application/json',
                    success: function (data, textStatus, jqXHR) {
                        console.log("success - textStatus   ".concat(JSON.stringify(textStatus, null, 4)));
                        console.log("success - found articles ".concat(JSON.stringify(data, null, 4)));
                        if (data !== null && data.length > 0) {
                            var articles = data;
                            that.displayArticles(articles);
                            $(".searchresult-text").text(articles.length + " Artikel gefunden");
                        }
                        else {
                            $(".searchresult-text").text("Suche nach '".concat(searchText, "\"' hat keine Artikel gefunden."));
                        }
                        var cnt = data.length;
                        console.log("success - found articles ".concat(cnt));
                    },
                    error: function (jqXHR, textStatus, errorThrown) {
                        console.log("error -   ".concat(textStatus));
                        console.log("error -   ".concat(errorThrown));
                        console.log(textStatus);
                        console.log(errorThrown);
                    }
                });
            });
        };
        SearchResultComp.prototype.getArticleView = function () {
            return " \n            <div class=\"col\">\n                <div class=\"card\">\n                    <div class=\"row g-0\">\n                        <div class=\"col-md-4\">\n                            <div class=\"bd-placeholder-img\" width=\"100%\" height=\"250\"  role=\"img\" aria-label=\"Placeholder: Image\"  focusable=\"false\">\n                                <img class=\"card-img-top article-image\" style=\"height: 225px; width: 100%; display: block;\"/>                               \n                            </div>\n                        </div>\n                        <div class=\"col-md-8\">\n                            <div class=\"card-body\">\n                                <h5 class=\"card-title article-name\" ></h5>\n                                <p class=\"card-text description\"></p>\n                                <p class=\"card-text article-code\"><small class=\"text-muted\"> </small></p>\n                            </div>\n                        </div>\n                    </div>\n                </div>\n            </div>\n           ";
        };
        SearchResultComp.prototype.createSearchResultApp = function () {
            // @ts-ignore
            return "\n            <div class=\"d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom\">\n                <h1 class=\"h2 searchresult-text\">Dashboard</h1> \n            </div>\n            <div class=\"album py-5 bg-light\">\n                <div id=\"searchResult\"  class=\"container\">\n                    <div id=\"searchResultArticleList\" class=\"row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3\">\n                   \n                    </div>\n                </div>\n           </div>";
        };
        return SearchResultComp;
    }(HTMLElement));
    window.customElements.define('search-result-comp', SearchResultComp);
});
