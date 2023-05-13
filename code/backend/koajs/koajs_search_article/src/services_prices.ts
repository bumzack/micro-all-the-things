import {
    Configuration,
    ConfigurationParameters,
    DefaultApi,
    GetPricesForMoviesRequest,
    PriceEntry,
    SearchPricesRequest
} from "./generated-clients/priceservice";

export const find_prices = async (tconsts: Array<string>): Promise<Array<PriceEntry>> => {
    const configParams: ConfigurationParameters = {
        basePath: 'http://localhost:58800',
    };

    const apiConfig = new Configuration(configParams);
    const api = new DefaultApi(apiConfig);

    let searchPricesRequest: SearchPricesRequest = {
        movieTconst: tconsts
    };
    let req: GetPricesForMoviesRequest = {
        searchPricesRequest: searchPricesRequest

    };
    return await api.getPricesForMovies(req);
}