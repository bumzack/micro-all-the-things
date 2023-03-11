/// <reference path ="../node_modules/@types/jquery/JQuery.d.ts"/>
/// <amd-dependency>

import {Category} from "./../../common/dtos";
import {doCategoryClickEvent, idSearchResultApp, tagCategoryCode, tagCategoryDepth} from "./../../common/const";

class CategoryComp extends HTMLElement {

    constructor() {
        super();
        console.info("constructor CategoryComp")
    }

    connectedCallback() {
        this.innerHTML += this.getCategory();
        this.loadRootCategories();
    }

    private addClickHandler() {
        console.log(`adding clickhandlers`);
        $(".category-name").on("click", (e) => {
            const clickedElement = $(event.target);

            const depth = $( clickedElement ).attr(tagCategoryDepth);
            const code = $( clickedElement ).attr(tagCategoryCode);
            console.log(`click on category ${code} has depth ${depth}`);

            document.getElementById(idSearchResultApp).dispatchEvent(
                new CustomEvent(doCategoryClickEvent, {
                    detail: {
                        tagCategoryCode: code
                    }
                })
            );
        });
    }

    private loadRootCategories() {
        const url = "http://localhost:8200/solr/search/category/root";
        console.log(`load root categories GET to url ${url} with request data`);

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
                    const rootCategories = data.items as Array<Category>;
                    that.displayCategories(rootCategories);
                } else {
                    console.log("no root categories found");
                }
                const cnt = data.items.length;
                console.log(`success - found articles ${cnt}`);

                that.addClickHandler();
            },
            error: function (jqXHR, textStatus, errorThrown) {
                console.log(`error -   ${textStatus}`);
                console.log(`error -   ${errorThrown}`);
                console.log(textStatus);
                console.log(errorThrown);
            }
        });
    }

    displayCategories = (categories: Array<Category>): void => {
        $("#searchResultArticleList").empty();
        categories.forEach(category => {
            console.log(`adding category ${category.code}`);
            const htmlCategory = $(this.getCategoryEntry());
            $(htmlCategory).text(category.name.slice(0, 20));
            $(htmlCategory).attr(tagCategoryDepth, "0");
            $(htmlCategory).attr(tagCategoryCode, category.code);
            $("#categoryTree").append(htmlCategory);
        });
    }

    // @ts-ignore
    getCategory() {
        console.info("getcategory link")
        return `
            <div id="categoryTree" class="bd-heading sticky-xl-top align-self-start mt-5 mb-3 mt-xl-0 mb-xl-2">
            </div>
        `;
    }

    getCategoryEntry() {
        return `
                 <p class="d-flex align-items-center category-name">as</p>
             `;
    }
}

window.customElements.define('category-comp', CategoryComp);