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
define("searchapp/src/main", ["require", "exports", "common/const"], function (require, exports, const_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    var SearchComp = /** @class */ (function (_super) {
        __extends(SearchComp, _super);
        function SearchComp() {
            var _this = _super.call(this) || this;
            console.log("constructor called");
            return _this;
        }
        SearchComp.prototype.createSearchApp = function () {
            return "<input id=\"searchtext\"  type=\"search\" class=\"form-control\" placeholder=\"Search...\" aria-label=\"Search\"/>";
        };
        SearchComp.prototype.connectedCallback = function () {
            console.log("search app callback");
            this.innerHTML += this.createSearchApp();
            var txt = document.getElementById("searchtext");
            $("#searchtext").keydown(function (event) {
                if (event.key === "Enter") {
                    // @ts-ignore
                    if (txt.value != null) {
                        // @ts-ignore
                        console.log("sending custom event with  search text: '", txt.value, "'");
                        document.getElementById(const_1.idSearchResultApp).dispatchEvent(new CustomEvent(const_1.doSearchClickEvent, {
                            detail: {
                                "searchText": txt.value
                            }
                        }));
                    }
                }
            });
        };
        return SearchComp;
    }(HTMLElement));
    window.customElements.define('search-comp', SearchComp);
});
define("common/dtos", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
});
