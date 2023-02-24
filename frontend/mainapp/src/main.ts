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

this.addEventListener('doSearch', e => {
    console.log("main: event 'doSearch' received: ", JSON.stringify(e, null, 5));
    // @ts-ignore
    console.log("main: event 'doSearch' received. searchtext ", JSON.stringify(e.detail, null, 5));
// @ts-ignore
    this.innerHTML = this.getCartDoc(e.detail);
});

console.log("main ");