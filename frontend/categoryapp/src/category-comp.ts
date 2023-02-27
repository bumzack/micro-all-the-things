/// <reference path ="../node_modules/@types/jquery/jquery.d.ts"/>

class CategoryComp extends HTMLElement {

    constructor() {
        super();
        console.info("constructor CategoryComp")
    }

    connectedCallback() {
        this.innerHTML += this.getCategory();
    }

    // @ts-ignore
    getCategory() {
        console.info("getCartDoc")
        // return `<a class="d-flex align-items-center" href="../components/card/">Documentation</a>`;
        return 'hahahaha';
    }
}

window.customElements.define('category-comp', CategoryComp);