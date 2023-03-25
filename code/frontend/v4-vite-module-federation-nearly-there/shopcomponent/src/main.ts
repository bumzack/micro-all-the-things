import jQuery from 'jquery';

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery: any;
    }
}

window.$ = window.jQuery = jQuery;

async function init() {
    console.log("xxxxxxxxxxxx  xxxxx   car app callback");
    let app1 = await import("remoteApp/App1");
    let cartApp = await import("cartApp/CartApp");
    const a = app1.default();
    console.log("axxxxxaaa ", a);
    const b = cartApp.default();
    console.log("xxxxx   bbbbb ", b);
}


class ShopComponent extends HTMLElement {

    constructor() {
        super();
        console.log("shop componentn constructor called");
    }

    createShopApp() {
        const root = $("#root");
        console.log("createShopApp");
        root.append("ShopComponent JQUERY hello from jquery slkfjslj lsjf jsldjf ljsfl");
        return `
            <div class="container-fluid">
                <div class="row-lg">
                    left column
                    <div>before</div>
                    <search-component></search-component>
                    <cart-component></cart-component>
                    <div>after</div>
                </div>
                <div class="row-lg">
                    right column
                </div>
            </div>
        `;
    }

    connectedCallback() {
        console.log("before init ShopApp");
        init()
        console.log("after init ShopApp");

        this.innerHTML += this.createShopApp();
    }
}

window.customElements.define('shop-component', ShopComponent);

console.log("hallo shop-component")

export let ShopApp: () => string = function () {
    return `shop app!`;
};

export default ShopApp;


