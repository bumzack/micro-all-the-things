/// <reference path ="../node_modules/@types/jquery/JQuery.d.ts"/>
/// <reference path ="../../../common/const.d.ts"/>


// https://isotropic.co/how-to-fix-the-property-does-not-exist-on-type-window-error-in-typescript/
import KeyDownEvent = JQuery.KeyDownEvent;

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery:any;
    }
}

window.$ = window.jQuery = jQuery;

import {SearchText} from "../../../common/dtos";

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

        jQuery("#searchtext").keydown((event:KeyDownEvent) => {
            if (event.key === "Enter") {
                if (txt.value != null) {
                    // @ts-ignore
                    console.log("sending custom event with  search text: '", txt.value, "'");

                    const doc = document.getElementById(ID_SEARCH_RESULT_APP);
                    if (doc !== null ) {
                        doc.dispatchEvent(
                            new CustomEvent<SearchText>(DO_SEARCH_EVENT, {
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
