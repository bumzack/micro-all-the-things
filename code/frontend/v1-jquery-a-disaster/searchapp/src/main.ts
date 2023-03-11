/// <reference path ="../node_modules/@types/jquery/jquery.d.ts"/>
/// <amd-dependency>


import {doSearchClickEvent, idSearchResultApp} from ".,/../common/const";

class SearchComp extends HTMLElement {

    constructor() {
        super();
        console.log("constructor called");
    }

    createSearchApp() {
        return `<input id="searchtext"  type="search" class="form-control" placeholder="Search..." aria-label="Search"/>`;
    }

    connectedCallback() {
        console.log("search app callback");
        this.innerHTML += this.createSearchApp();

        const txt = <HTMLInputElement>document.getElementById("searchtext");

        $("#searchtext").keydown((event: JQuery.Event) => {
            if (event.key === "Enter") {
                // @ts-ignore
                if (txt.value != null) {
                    // @ts-ignore
                    console.log("sending custom event with  search text: '", txt.value, "'");

                    document.getElementById(idSearchResultApp).dispatchEvent(
                        new CustomEvent(doSearchClickEvent, {
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