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
            // let url = "http://search.rust.bumzack.at/api/v1/solr/article";
            //  let url = "http://proxy.proxythingi.at/rust/solr/search"
            let url = "http://searchindex.rust.bumzack.at/api/v1/solr/searchindex/search";

            const customer: SearchCustomer = {
                customer_id: 1,
                jwt: "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9",
            };

            const req: SearchArticleRequest = {
                q: txt,
                offset: 0,
                limit: 50,
                customer: customer,
            };

            console.log(`sending request  to url ${url}. req  ${JSON.stringify(req, null, 4)} `);

            var ajxReq = jquery.ajax(url, {
                type: "POST",
                data: JSON.stringify(req),
                contentType: 'application/json',
                dataType: 'json',
                timeout: 300,
                success: function (data, status, jqXhr) {
                    console.log(`status   ${JSON.stringify(status)}`);
                    const movies = data as SearchArticleResponse;
                    console.log(`movies  ${JSON.stringify(movies, null, 4)} `);

                    if (movies.articles !== undefined) {
                        if (movies.articles.length > 0) {
                            jquery("#search_results").innerHTML("no movies found");
                        } else {
                            jquery("#search_results").innerHTML(`found ${movies.articles.length} movies`);
                        }
                    } else {
                        jquery("#search_results").text(`ups - got an empty result`);
                    }
                },
                fail: function (e) {
                    console.error(`fail error requesting the movies. ${e}`);
                },

            });


        }
    });
})


export {};


