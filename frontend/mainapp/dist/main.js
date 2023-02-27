this.addEventListener('doSearch', e => {
    console.log("main: event 'doSearch' received: ", JSON.stringify(e, null, 5));
    console.log("main: event 'doSearch' received. searchtext ", JSON.stringify(e.detail, null, 5));
});
console.log("main ");
//# sourceMappingURL=main.js.map