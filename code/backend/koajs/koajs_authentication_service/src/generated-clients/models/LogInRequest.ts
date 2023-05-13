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

import {exists} from '../runtime';

/**
 *
 * @export
 * @interface LogInRequest
 */
export interface LogInRequest {
    /**
     *
     * @type {string}
     * @memberof LogInRequest
     */
    email?: string;
    /**
     *
     * @type {string}
     * @memberof LogInRequest
     */
    password?: string;
}

/**
 * Check if a given object implements the LogInRequest interface.
 */
export function instanceOfLogInRequest(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function LogInRequestFromJSON(json: any): LogInRequest {
    return LogInRequestFromJSONTyped(json, false);
}

export function LogInRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): LogInRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'email': !exists(json, 'email') ? undefined : json['email'],
        'password': !exists(json, 'password') ? undefined : json['password'],
    };
}

export function LogInRequestToJSON(value?: LogInRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'email': value.email,
        'password': value.password,
    };
}

