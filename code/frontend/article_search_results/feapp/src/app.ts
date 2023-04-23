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
            const url = "http://proxy.proxythingi.at/rust/solr/search"

            const customer: SearchCustomer = {
                customer_id: 1,
                jwt: "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9",
            };

            const req: SearchArticleRequest = {
                q: txt,
                offset: 0,
                limit: 5,
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
                const response= await fetch(url, {
                    method: "POST", // *GET, POST, PUT, DELETE, etc.
                    mode: "cors", // no-cors, *cors, same-origin
                    cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
                    credentials: "same-origin", // include, *same-origin, omit
                    headers: {
                        "Content-Type": "application/json",
                        'Access-Control-Expose-Headers': 'x-duration,x-provided-by'
                    },
                    redirect: "follow", // manual, *follow, error
                    referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
                    body: JSON.stringify(data), // body data type must match "Content-Type" header
                });
                return [await response.json(),  response.headers] ; // parses JSON response into native JavaScript objects
            }

            postData(url, req).then(([data,headers]) => {
                console.log(data); // JSON data parsed by `data.json()` call
                console.log(headers); // JSON data parsed by `data.json()` call

                for (var pair of headers.entries()) {
                    console.log(pair[0]+ ': '+ pair[1]);
                }


                const dur = headers.get("x-duration");
                const provided_by = headers.get("x-provided-by");
                console.log(`proxythingi backend  ${provided_by},  duration  ${dur} `);

                const movies = data as SearchArticleResponse;
                console.log(`movies.len ${movies.articles.length} `);

                jquery("#search_results").empty();

                movies.articles.forEach(a=> {
                    const elem = article_template(a);
                    jquery("#search_results").append(elem);
                })
            });
        }
    });
})


const article_template = (article:ArticleSearchResult): string => {
    const title = article.article.titles.join(" / ");
    const characters ="Characters: " +  article.article.characters.join(" / ");
    const act ="Actresses / Actors : " +  article.article.actors.join(" / ");


    let price:string;
    if (article.customer_price!== undefined) {
          price = `SRP € <sr>${article.price.toFixed(2)}</sr>, your price: € ${article.customer_price.toFixed(2)}`;
    }else {
          price = `SRP € ${article.price} `;
    }

    return `   
        <div class="col">
            <div class="card h-100">
                 <div class="card-body">
                    <h5 class="card-title">${title}</h5>
                    <p class="card-text">${act}</p>
                    <p class="card-text">${characters}</p>
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
    `;

}
export {};


