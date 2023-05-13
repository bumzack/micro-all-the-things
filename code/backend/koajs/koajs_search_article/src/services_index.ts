import {
    Configuration,
    ConfigurationParameters,
    DefaultApi,
    MovieSearchResult,
    SearchDocsRequest,
    SearchMovieIndexRequest
} from "./generated-clients/searchindexservice";

export const search_movies = async (q: string, offset: number, limit: number): Promise<MovieSearchResult> => {
    const configParams: ConfigurationParameters = {
        basePath: 'http://localhost:58320',
    };

    const apiConfig = new Configuration(configParams);
    const api = new DefaultApi(apiConfig);
    let search_movie_index_request: SearchMovieIndexRequest = {
        q: q,
        offset: offset,
        limit: limit,
    };
    let req: SearchDocsRequest = {
        searchMovieIndexRequest: search_movie_index_request,
    };
    console.log(`search_movies   request ${JSON.stringify(req)} `);
    return await api.searchDocs(req);
}