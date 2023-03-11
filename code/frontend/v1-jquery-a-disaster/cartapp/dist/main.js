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
/// <amd-dependency>
var CartComp = /** @class */ (function (_super) {
    __extends(CartComp, _super);
    function CartComp() {
        var _this = _super.call(this) || this;
        console.info("constructor");
        _this.addEventListener('cartLoaded', function (e) {
            // @ts-ignore
            _this.innerHTML = _this.getCartDoc(e.detail);
        });
        return _this;
    }
    // @ts-ignore
    CartComp.prototype.getCartDoc = function (detail) {
        console.info("getCartDoc");
        return "<ol class=\"list-group\">".concat(this.getLineItems(detail), "</ol>");
    };
    // @ts-ignore
    CartComp.prototype.getLineItems = function (detail) {
        console.info("getLineItems");
        var lineItems = '';
        for (var _i = 0, detail_1 = detail; _i < detail_1.length; _i++) {
            var s = detail_1[_i];
            lineItems += "<li class=\"list-group-item\"><img src=\"img/".concat(s, ".jpg\" alt=\"").concat(s, "\" height=\"20\" width=\"20\">").concat(s, "</li>");
        }
        return lineItems;
    };
    return CartComp;
}(HTMLElement));
window.customElements.define('cart-comp', CartComp);
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
