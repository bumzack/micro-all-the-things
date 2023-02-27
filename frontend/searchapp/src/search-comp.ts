/// <reference path ="../node_modules/@types/jquery/jquery.d.ts"/>

class SearchComp extends HTMLElement {

    constructor() {
        super();
        console.log("constructor called");

        this.addEventListener('cartLoaded', e => {
            // @ts-ignore
            this.innerHTML = this.getCartDoc(e.detail);
        });
    }

    createSearchApp() {
        return `<input id="searchtext"  grg="lala" type="search" class="form-control" placeholder="Search..." aria-label="Search"/>`;
    }

    connectedCallback() {
        this.innerHTML += this.createSearchApp();

        const txt = <HTMLInputElement>document.getElementById("searchtext");

        $("#searchtext").keydown((event: JQuery.Event) => {
            if (event.key === "Enter") {
                // @ts-ignore
                if (txt.value != null) {
                    // @ts-ignore
                    console.log("sending custom event with  search text: '", txt.value, "'");

                    document.getElementById("searchapp").dispatchEvent(
                        new CustomEvent('doSearch', {
                            detail: {
                                "searchText": txt.value
                            }
                        })
                    );
                }
            }
        });
    }
}

window.customElements.define('search-comp', SearchComp);