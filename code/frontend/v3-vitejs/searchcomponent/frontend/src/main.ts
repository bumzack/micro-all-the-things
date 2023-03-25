/// <reference path ="../node_modules/@types/jquery/JQuery.d.ts"/>


// https://isotropic.co/how-to-fix-the-property-does-not-exist-on-type-window-error-in-typescript/
declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery:any;
    }
}

window.$ = window.jQuery = jQuery;

import {DO_SEARCH_EVENT, ID_SEARCH_RESULT_APP} from "../../../common/const";

class SearchComp extends HTMLElement {

    constructor() {
        super();
        console.log("search component constructor called");
    }

    createSearchApp() {
        return `<input id="searchtext"  type="search" class="form-control" placeholder="Search..." aria-label="Search"/>`;
    }

    connectedCallback() {
        console.log("search component callback");
        this.innerHTML += this.createSearchApp();

        const txt = <HTMLInputElement>document.getElementById("searchtext");

        jQuery("#searchtext").keydown((event) => {
            if (event.key === "Enter") {
                // @ts-ignore
                if (txt.value != null) {
                    // @ts-ignore
                    console.log("sending custom event with  search text: '", txt.value, "'");

                    const doc = document.getElementById(ID_SEARCH_RESULT_APP);
                    if (doc !== null ) {
                        doc.dispatchEvent(
                            new CustomEvent(DO_SEARCH_EVENT, {
                                detail: {
                                    "searchText": txt.value
                                }
                            })
                        );
                    }
                }
            }
        });
    }
}

window.customElements.define('search-component', SearchComp);
