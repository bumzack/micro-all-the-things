/* tslint:disable */
/* eslint-disable */
/**
 * Authentication Service
 * Authentication Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {AuthenticationEntry, LogInRequest, LogOutRequest,} from '../models';
import {AuthenticationEntryFromJSON, LogInRequestToJSON, LogOutRequestToJSON,} from '../models';

export interface LoggedinRequest {
    customerId: number;
}

export interface LoginRequest {
    logInRequest?: LogInRequest;
}

export interface LogoutRequest {
    logOutRequest?: LogOutRequest;
}

/**
 *
 */
export class DefaultApi extends runtime.BaseAPI {

    /**
     */
    async loggedinRaw(requestParameters: LoggedinRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<AuthenticationEntry>> {
        if (requestParameters.customerId === null || requestParameters.customerId === undefined) {
            throw new runtime.RequiredError('customerId', 'Required parameter requestParameters.customerId was null or undefined when calling loggedin.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/authenticated/{customerId}`.replace(`{${"customerId"}}`, encodeURIComponent(String(requestParameters.customerId))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => AuthenticationEntryFromJSON(jsonValue));
    }

    /**
     */
    async loggedin(requestParameters: LoggedinRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<AuthenticationEntry> {
        const response = await this.loggedinRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async loginRaw(requestParameters: LoginRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<AuthenticationEntry>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/authentication/login`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: LogInRequestToJSON(requestParameters.logInRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => AuthenticationEntryFromJSON(jsonValue));
    }

    /**
     */
    async login(requestParameters: LoginRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<AuthenticationEntry> {
        const response = await this.loginRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async logoutRaw(requestParameters: LogoutRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<AuthenticationEntry>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/authentication/logout`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: LogOutRequestToJSON(requestParameters.logOutRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => AuthenticationEntryFromJSON(jsonValue));
    }

    /**
     */
    async logout(requestParameters: LogoutRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<AuthenticationEntry> {
        const response = await this.logoutRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
