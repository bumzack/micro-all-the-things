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
define("categoryapp/src/main", ["require", "exports", "common/const"], function (require, exports, const_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    var CategoryComp = /** @class */ (function (_super) {
        __extends(CategoryComp, _super);
        function CategoryComp() {
            var _this = _super.call(this) || this;
            _this.displayCategories = function (categories) {
                $("#searchResultArticleList").empty();
                categories.forEach(function (category) {
                    console.log("adding category ".concat(category.code));
                    var htmlCategory = $(_this.getCategoryEntry());
                    $(htmlCategory).text(category.name.slice(0, 20));
                    $(htmlCategory).attr(const_1.tagCategoryDepth, "0");
                    $(htmlCategory).attr(const_1.tagCategoryCode, category.code);
                    $("#categoryTree").append(htmlCategory);
                });
            };
            console.info("constructor CategoryComp");
            return _this;
        }
        CategoryComp.prototype.connectedCallback = function () {
            this.innerHTML += this.getCategory();
            this.loadRootCategories();
        };
        CategoryComp.prototype.addClickHandler = function () {
            console.log("adding clickhandlers");
            $(".category-name").on("click", function (e) {
                var clickedElement = $(event.target);
                var depth = $(clickedElement).attr(const_1.tagCategoryDepth);
                var code = $(clickedElement).attr(const_1.tagCategoryCode);
                console.log("click on category ".concat(code, " has depth ").concat(depth));
                document.getElementById(const_1.idSearchResultApp).dispatchEvent(new CustomEvent(const_1.doCategoryClickEvent, {
                    detail: {
                        tagCategoryCode: code
                    }
                }));
            });
        };
        CategoryComp.prototype.loadRootCategories = function () {
            var url = "http://localhost:8200/solr/search/category/root";
            console.log("load root categories GET to url ".concat(url, " with request data"));
            var that = this;
            $.ajax({
                url: url,
                type: "GET",
                cache: false,
                async: true,
                processData: false,
                contentType: 'application/json',
                success: function (data, textStatus, jqXHR) {
                    console.log("success - textStatus   ".concat(JSON.stringify(textStatus, null, 4)));
                    console.log("success - found categories ".concat(JSON.stringify(data, null, 4)));
                    if (data.items !== null && data.items.length > 0) {
                        var rootCategories = data.items;
                        that.displayCategories(rootCategories);
                    }
                    else {
                        console.log("no root categories found");
                    }
                    var cnt = data.items.length;
                    console.log("success - found articles ".concat(cnt));
                    that.addClickHandler();
                },
                error: function (jqXHR, textStatus, errorThrown) {
                    console.log("error -   ".concat(textStatus));
                    console.log("error -   ".concat(errorThrown));
                    console.log(textStatus);
                    console.log(errorThrown);
                }
            });
        };
        // @ts-ignore
        CategoryComp.prototype.getCategory = function () {
            console.info("getcategory link");
            return "\n            <div id=\"categoryTree\" class=\"bd-heading sticky-xl-top align-self-start mt-5 mb-3 mt-xl-0 mb-xl-2\">\n            </div>\n        ";
        };
        CategoryComp.prototype.getCategoryEntry = function () {
            return "\n                 <p class=\"d-flex align-items-center category-name\">as</p>\n             ";
        };
        return CategoryComp;
    }(HTMLElement));
    window.customElements.define('category-comp', CategoryComp);
});
