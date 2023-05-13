"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.search_movies = void 0;
const searchindexservice_1 = require("./generated-clients/searchindexservice");
const search_movies = async (q, offset, limit) => {
    const configParams = {
        basePath: 'http://localhost:58320',
    };
    const apiConfig = new searchindexservice_1.Configuration(configParams);
    const api = new searchindexservice_1.DefaultApi(apiConfig);
    let search_movie_index_request = {
        q: q,
        offset: offset,
        limit: limit,
    };
    let req = {
        searchMovieIndexRequest: search_movie_index_request,
    };
    console.log(`search_movies   request ${JSON.stringify(req)} `);
    return await api.searchDocs(req);
};
exports.search_movies = search_movies;
//# sourceMappingURL=services_index.js.map