// document.getElementById("productsApp").addEventListener('click', e => {
//     console.log("event selectedFruit", JSON.stringify(e, null, 4));
//     // @ts-ignore
//     if (e.detail != null && e.detail.length >= 0) {
//         // @ts-ignore
//         document.getElementById("cartapp").dispatchEvent(
//             new CustomEvent('cartLoaded', {
//                 // @ts-ignore
//                 detail: e.detail
//             })
//         );
//         let elem = document.getElementById("itemsCount");
//         let count = elem.innerText;
//         // @ts-ignore
//         elem.innerText = '' + e.detail.length;
//     }
// });
/// <amd-dependency>
this.addEventListener('doSearch', function (e) {
    console.log("main: event 'doSearch' received: ", JSON.stringify(e, null, 5));
    // @ts-ignore
    console.log("main: event 'doSearch' received. searchtext ", JSON.stringify(e.detail, null, 5));
    // @ts-ignore
});
console.log("main ");
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
