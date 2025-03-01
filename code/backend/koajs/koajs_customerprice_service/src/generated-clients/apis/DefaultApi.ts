/* tslint:disable */
/* eslint-disable */
/**
 * CustomerPrice Service
 * CustomerPrice Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {CustomerPriceEntry,} from '../models';
import {CustomerPriceEntryFromJSON,} from '../models';

export interface CustomerPricesRequest {
    customerId: number;
}

/**
 *
 */
export class DefaultApi extends runtime.BaseAPI {

    /**
     */
    async customerPricesRaw(requestParameters: CustomerPricesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<CustomerPriceEntry>>> {
        if (requestParameters.customerId === null || requestParameters.customerId === undefined) {
            throw new runtime.RequiredError('customerId', 'Required parameter requestParameters.customerId was null or undefined when calling customerPrices.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/customerprices/{customerId}`.replace(`{${"customerId"}}`, encodeURIComponent(String(requestParameters.customerId))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(CustomerPriceEntryFromJSON));
    }

    /**
     */
    async customerPrices(requestParameters: CustomerPricesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<CustomerPriceEntry>> {
        const response = await this.customerPricesRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
