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
var ProductsComp = /** @class */ (function (_super) {
    __extends(ProductsComp, _super);
    function ProductsComp() {
        var _this = _super.call(this) || this;
        console.log("construtor Productscomp");
        return _this;
        // this.addEventListener('change', e => {
        //     // @ts-ignore
        //     if (e.target.type === "checkbox") {
        //         let atleastOneChecked = false;
        //         for (let el of this.getElementsByTagName('input')) {
        //             if (el.checked) {
        //                 atleastOneChecked = true;
        //                 break;
        //             }
        //         }
        //         //document.getElementById("productsCompBtn").disabled = !atleastOneChecked;
        //     }
        // });
        // this.addEventListener('click', e => {
        //     console.info("addEventListener")
        //     // @ts-ignore
        //     if (e.target.type === "submit") {
        //         let selectedFruits = new Array();
        //         for (let el of this.getElementsByTagName('input')) {
        //             if (el.checked) {
        //                 selectedFruits.push(el.getAttribute("value"));
        //             }
        //         }
        //
        //         if (selectedFruits.length >= 0) {
        //             this.dispatchEvent(
        //                 new CustomEvent('selectedFruits', {
        //                     detail: selectedFruits
        //                 })
        //             );
        //         }
        //     }
        // });
    }
    ProductsComp.prototype.connectedCallback = function () {
        this.innerHTML += this.createProductsApp();
    };
    ProductsComp.prototype.createProductsApp = function () {
        return "<div>\n        <div class=\"row row-cols-3\">\n            <div class=\"col-sm\"><img src=\"img/apple.jpg\" height=\"150\" width=\"150\"><br />\n            <span style=\"color: black;\">Apple $1.00/lb</span> \n            <input type=\"checkbox\" name=\"fruits\" value=\"apple\">\n            </div>\n            <div class=\"col-sm\"><img src=\"img/kiwi.jpg\" height=\"150\" width=\"150\"><br />\n            <span style=\"color: black;\">Kiwi $2.00/lb</span> <input type=\"checkbox\" name=\"fruits\" value=\"kiwi\">\n            </div>\n            <div class=\"col-sm\"><img src=\"img/orange.jpg\" height=\"150\" width=\"150\"><br />\n            <span style=\"color: black;\">Orange $1.00/lb</span> <input type=\"checkbox\" name=\"fruits\" value=\"orange\">\n            </div>\n            </div>\n        </div>\n        <div class=\"row row row-cols-3\"><div class=\"col-sm\">&nbsp;</div></div>\n        <div class=\"row row-cols-3\"><div class=\"col-sm d-flex justify-content-center\">\n        <p><button id=\"productsCompBtn\" class=\"btn btn-primary\">Add to Cart</button></p></div></div>";
    };
    return ProductsComp;
}(HTMLElement));
window.customElements.define('products-comp', ProductsComp);
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
