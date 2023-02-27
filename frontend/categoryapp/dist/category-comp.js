class CategoryComp extends HTMLElement {
    constructor() {
        super();
        this.displayCategories = (categories) => {
            $("#searchResultArticleList").empty();
            categories.forEach(category => {
                console.log(`adding category ${category.code}`);
                const htmlCategory = $(this.getCategoryEntry());
                $(htmlCategory).text(category.name.slice(0, 20));
                $("#categoryTree").append(htmlCategory);
            });
        };
        console.info("constructor CategoryComp");
    }
    connectedCallback() {
        this.innerHTML += this.getCategory();
        const url = "http://localhost:8200/solr/search/category/root";
        console.log(`sending GET to url ${url} with request data`);
        const that = this;
        $.ajax({
            url: url,
            type: "GET",
            cache: false,
            async: true,
            processData: false,
            contentType: 'application/json',
            success: function (data, textStatus, jqXHR) {
                console.log(`success - textStatus   ${JSON.stringify(textStatus, null, 4)}`);
                console.log(`success - found categories ${JSON.stringify(data, null, 4)}`);
                if (data.items !== null && data.items.length > 0) {
                    const rootCategories = data.items;
                    that.displayCategories(rootCategories);
                }
                else {
                    console.log("no root categories found");
                }
                const cnt = data.length;
                console.log(`success - found articles ${cnt}`);
            },
            error: function (jqXHR, textStatus, errorThrown) {
                console.log(`error -   ${textStatus}`);
                console.log(`error -   ${errorThrown}`);
                console.log(textStatus);
                console.log(errorThrown);
            }
        });
    }
    getCategory() {
        console.info("getcategory link");
        return `
            <div id="categoryTree" class="bd-heading sticky-xl-top align-self-start mt-5 mb-3 mt-xl-0 mb-xl-2">
            </div>
        `;
    }
    getCategoryEntry() {
        return `
                 <a class="d-flex align-items-center category-name" href="../components/card/">as</a>
             `;
    }
}
window.customElements.define('category-comp', CategoryComp);
export {};
//# sourceMappingURL=category-comp.js.map