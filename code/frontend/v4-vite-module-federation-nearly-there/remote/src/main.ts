import jQuery from 'jquery';

window.$ = window.jQuery = jQuery;

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery: any;
    }
}

class SearchComp extends HTMLElement {

    constructor() {
        super();
        console.log("search component constructor called");
    }

    createSearchApp() {
        const root = $("#root");
        root.append("SearchComonent JQUERY hello from jquery slkfjslj lsjf jsldjf ljsfl");
        return `<input id="searchtext"  type="search" class="form-control" placeholder="Search..." aria-label="Search"/>`;
    }

    connectedCallback() {
        console.log("search component callback");
        this.innerHTML += this.createSearchApp();
    }
}

window.customElements.define('search-component', SearchComp);

console.log("halllo searchcomponent")

export let App1: () => string = function () {
    return `APP2 22 2 2 2 2222 2 2 2 22 2 for the  double Win!`;
};


export default App1;
