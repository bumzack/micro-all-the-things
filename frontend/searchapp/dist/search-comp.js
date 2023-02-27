class SearchComp extends HTMLElement {
    constructor() {
        super();
        console.log("constructor called");
        this.addEventListener('cartLoaded', e => {
            this.innerHTML = this.getCartDoc(e.detail);
        });
    }
    createSearchApp() {
        return `<input id="searchtext"  grg="lala" type="search" class="form-control" placeholder="Search..." aria-label="Search"/>`;
    }
    connectedCallback() {
        this.innerHTML += this.createSearchApp();
        const txt = document.getElementById("searchtext");
        $("#searchtext").keydown((event) => {
            if (event.key === "Enter") {
                if (txt.value != null) {
                    console.log("sending custom event with  search text: '", txt.value, "'");
                    document.getElementById("searchapp").dispatchEvent(new CustomEvent('doSearch', {
                        detail: {
                            "searchText": txt.value
                        }
                    }));
                }
            }
        });
    }
}
window.customElements.define('search-comp', SearchComp);
//# sourceMappingURL=search-comp.js.map