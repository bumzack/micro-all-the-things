import jquery from "jquery";
import {SearchArticleRequest, SearchArticleResponse, SearchCustomer} from "./common";

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery: any;
    }
}

window.$ = window.jQuery = jquery;

jquery(document).ready(() => {
    console.log("yo");


    jquery("#searchMovie").keydown((event: KeyboardEvent) => {
        if (event.which == 13) {
            event.preventDefault();
            const txt = jquery("#searchMovie").val() as string;
            console.log(`return pressed     ${txt}  `);
            const url = "http://proxy.proxythingi.at/rust/solr/search"

            const customer: SearchCustomer = {
                customer_id: 1,
                jwt: "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9",
            };

            const req: SearchArticleRequest = {
                q: txt,
                offset: 0,
                limit: 1,
                customer: customer,
            };

            console.log(`sending request  to url ${url}. req  ${JSON.stringify(req, null, 4)} `);


            // https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
            async function postData(url = "", data = {}) {
                // Default options are marked with *
                const response = await fetch(url, {
                    method: "POST", // *GET, POST, PUT, DELETE, etc.
                    mode: "cors", // no-cors, *cors, same-origin
                    cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
                    credentials: "same-origin", // include, *same-origin, omit
                    headers: {
                        "Content-Type": "application/json",
                        // 'Content-Type': 'application/x-www-form-urlencoded',
                    },
                    redirect: "follow", // manual, *follow, error
                    referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
                    body: JSON.stringify(data), // body data type must match "Content-Type" header
                });
                return response.json(); // parses JSON response into native JavaScript objects
            }

            postData(url, req).then((data) => {
                console.log(data); // JSON data parsed by `data.json()` call
            });
        }
    });
})


export {};


