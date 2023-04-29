import jquery from "jquery";
import {ArticleSearchResult, SearchArticleRequest, SearchArticleResponse, SearchCustomer} from "./common";

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
            const url_prod = "http://proxy.proxythingi.at/webflux/solr/search"

            // rust
            // const url_prod = "http://search.rust.bumzack.at/api/v1/solr/article"
            // const url_prod = "http://search.rust.bumzack.at/api/v1/meili/article"

            // webflux
            // const url_prod = "http://search.webflux.bumzack.at/api/v1/solr/article"


            const url_local = "http://localhost:18600/api/v2/solr/article"
            const url = url_prod;

            const customer: SearchCustomer = {
                customer_id: 1,
                jwt: "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9",
            };

            const req: SearchArticleRequest = {
                q: txt,
                offset: 0,
                limit: 10,
                customer: customer,
            };

            console.log(`sending request  to url ${url}. req  ${JSON.stringify(req, null, 4)} `);

            // funny stuff demo

            // var ajxReq = jquery.ajax(url, {
            //     type: "POST",
            //     data: JSON.stringify(req),
            //     contentType: 'application/json',
            //     dataType: 'json',
            //     timeout: 300,
            //     success: function (data, status) {
            //         console.log(`status   ${JSON.stringify(status)}`);
            //         const movies = data as SearchArticleResponse;
            //         console.log(`movies  ${JSON.stringify(movies, null, 4)} `);
            //
            //         if (movies.articles !== undefined) {
            //             if (movies.articles.length > 0) {
            //                 jquery("#search_results").innerHTML("no movies found");
            //             } else {
            //                 jquery("#search_results").innerHTML(`found ${movies.articles.length} movies`);
            //             }
            //         } else {
            //             jquery("#search_results").text(`ups - got an empty result`);
            //         }
            //     },
            //     fail: function (e) {
            //         console.error(`fail error requesting the movies. ${e}`);
            //     },
            //
            // });
            //

            // https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
            async function postData(url = "", data = {}): Promise<[Response, Headers]> {
                // Default options are marked with *
                const response = await fetch(url, {
                    method: "POST", // *GET, POST, PUT, DELETE, etc.
                    mode: "cors", // no-cors, *cors, same-origin
                    cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
                    credentials: "same-origin", // include, *same-origin, omit
                    headers: {
                        "Content-Type": "application/json",
                     //   "Access-Control-Expose-Headers": "x-duration,x-provided-by,x-initiated-by,x-processed-by"
                        "Access-Control-Expose-Headers":"*",
                    },
                    // redirect: "follow", // manual, *follow, error
                    // referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
                    body: JSON.stringify(data), // body data type must match "Content-Type" header
                });
                for (const [h1, h2] of response.headers) {
                    console.log(h1 + ': ' + h2);
                }

                return [await response.json(), response.headers]; // parses JSON response into native JavaScript objects
            }

            postData(url, req).then(([data, headers]) => {
                console.log(data); // JSON data parsed by `data.json()` call
                console.log(headers); // JSON data parsed by `data.json()` call

                for (const header of headers.entries()) {
                    console.log(header[0] + ': ' + header[1]);
                }

                const dur = headers.get("x-duration");
                const provided_by = headers.get("x-provided-by");
                const initiated_by = headers.get("x-initiated-by");
                const processed_by = headers.get("x-processed-by");

                jquery("#dur").text(dur);
                jquery("#provided_by").text(provided_by);
                jquery("#initiated_by").text(initiated_by);
                jquery("#processed_by").text(processed_by);


                console.log(`proxythingi backend  ${provided_by},  duration  ${dur}    initiated_by  ${initiated_by}   processed_by  ${processed_by} `);

                const movies = data as SearchArticleResponse;
                console.log(`movies.len ${movies.articles.length} `);

                jquery("#search_results").empty();
                jquery("#search_results").text(`${movies.articles.length} movies found`);

                movies.articles.forEach(a => {
                    const elem = article_template(a);
                    jquery("#search_results").append(elem);
                })
            });
        }
    });
})


const article_template = (article: ArticleSearchResult): string => {
    let titles = "";
    if (article.article.titles != undefined) {
        titles = "Titles: " + article.article.titles.join(" / ");
    }

    let primary_title = "";
    if (article.article.primary_title != undefined) {
        primary_title = article.article.primary_title;
    }

    let original_title = "";
    if (article.article.original_title != undefined) {
        original_title = article.article.original_title;
    }

    let characters = "";
    if (article.article.characters != undefined) {
        characters = "Characters: " + article.article.characters.join(" / ");
    }

    let act = "";
    if (article.article.actors != undefined) {
        act = "Actors: " + article.article.actors.join(" / ");
    }

    let writers = "";
    if (article.article.writers != undefined) {
        writers = "Writers: " + article.article.writers.join(" / ");
    }

    let directors = "";
    if (article.article.directors != undefined) {
        directors = "Directors: " + article.article.directors.join(" / ");
    }

    let price: string;
    if (article.customer_price !== undefined && article.customer_price !== null) {
        price = `SRP € <sr>${article.price.toFixed(2)}</sr>, your price: € ${article.customer_price.toFixed(2)}`;
    } else {
        price = `SRP € ${article.price} `;
    }

    return `   
        <div class="col">
            <div class="card h-100">
                 <div class="card-body">
                    <h5 class="card-title">original title${original_title}</h5>
                    <h5 class="card-title">primary title${primary_title}</h5>
                    <p class="card-body">${titles}</p>
                    <p class="card-body">${act}</p>
                    <p class="card-body">${characters}</p>
                    <p class="card-body">${directors}</p>
                    <p class="card-body">${writers}</p>
                    <p class="card-body">ttconst ${article.article.tconst}</p>
                 </div>
                 <div class="card-footer">
                    <div class="row row-cols-2 row-cols-md-2 g-6">
                       <div class="col">
                            ${price}
                        </div>
                    <div class="col">
                        <a href="#" class="btn btn-primary">Add to Cart</a>
                    </div>
                 </div>
            </div>
        </div>
    </div>
    `
        ;

}
export {};


