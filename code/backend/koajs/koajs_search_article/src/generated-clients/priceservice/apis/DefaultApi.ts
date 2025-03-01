/* tslint:disable */
/* eslint-disable */
/**
 * Price Service
 * Price Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {PriceEntry, SearchPricesRequest,} from '../models';
import {PriceEntryFromJSON, SearchPricesRequestToJSON,} from '../models';

export interface GetPriceForMovieRequest {
    tconst: string;
}

export interface GetPricesForMoviesRequest {
    searchPricesRequest?: SearchPricesRequest;
}

/**
 *
 */
export class DefaultApi extends runtime.BaseAPI {

    /**
     */
    async getPriceForMovieRaw(requestParameters: GetPriceForMovieRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PriceEntry>> {
        if (requestParameters.tconst === null || requestParameters.tconst === undefined) {
            throw new runtime.RequiredError('tconst', 'Required parameter requestParameters.tconst was null or undefined when calling getPriceForMovie.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/price/{tconst}`.replace(`{${"tconst"}}`, encodeURIComponent(String(requestParameters.tconst))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PriceEntryFromJSON(jsonValue));
    }

    /**
     */
    async getPriceForMovie(requestParameters: GetPriceForMovieRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PriceEntry> {
        const response = await this.getPriceForMovieRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async getPricesForMoviesRaw(requestParameters: GetPricesForMoviesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<PriceEntry>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v2/prices`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: SearchPricesRequestToJSON(requestParameters.searchPricesRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(PriceEntryFromJSON));
    }

    /**
     */
    async getPricesForMovies(requestParameters: GetPricesForMoviesRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<PriceEntry>> {
        const response = await this.getPricesForMoviesRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
