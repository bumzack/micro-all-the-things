class CartComp extends HTMLElement {

    constructor() {
        super();
        console.info("constructor")
        this.addEventListener('cartLoaded', e => {
            // @ts-ignore
            this.innerHTML = this.getCartDoc(e.detail);
        });
    }

    // @ts-ignore
    getCartDoc(detail) {
        console.info("getCartDoc")
        return `<ol class="list-group">${this.getLineItems(detail)}</ol>`;
    }

    // @ts-ignore
    getLineItems(detail) {
        console.info("getLineItems")
        let lineItems = '';
        for (let s of detail) {
            lineItems += `<li class="list-group-item"><img src="img/${s}.jpg" alt="${s}" height="20" width="20">${s}</li>`;
        }
        return lineItems;
    }
}
window.customElements.define('cart-comp', CartComp);