class CategoryComp extends HTMLElement {
    constructor() {
        super();
        console.info("constructor CategoryComp");
    }
    connectedCallback() {
        this.innerHTML += this.getCategory();
    }
    getCategory() {
        console.info("getCartDoc");
        return 'hahahaha';
    }
}
window.customElements.define('category-comp', CategoryComp);
//# sourceMappingURL=category-comp.js.map