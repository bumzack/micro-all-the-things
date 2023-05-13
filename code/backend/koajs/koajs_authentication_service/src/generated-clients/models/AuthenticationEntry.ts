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
 * @interface AuthenticationEntry
 */
export interface AuthenticationEntry {
    /**
     *
     * @type {number}
     * @memberof AuthenticationEntry
     */
    id?: number;
    /**
     *
     * @type {number}
     * @memberof AuthenticationEntry
     */
    customerId?: number;
    /**
     *
     * @type {string}
     * @memberof AuthenticationEntry
     */
    jwt?: string;
    /**
     *
     * @type {Date}
     * @memberof AuthenticationEntry
     */
    loggedIn?: Date;
    /**
     *
     * @type {Date}
     * @memberof AuthenticationEntry
     */
    loggedOut?: Date;
    /**
     *
     * @type {Date}
     * @memberof AuthenticationEntry
     */
    created?: Date;
}

/**
 * Check if a given object implements the AuthenticationEntry interface.
 */
export function instanceOfAuthenticationEntry(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function AuthenticationEntryFromJSON(json: any): AuthenticationEntry {
    return AuthenticationEntryFromJSONTyped(json, false);
}

export function AuthenticationEntryFromJSONTyped(json: any, ignoreDiscriminator: boolean): AuthenticationEntry {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'id': !exists(json, 'id') ? undefined : json['id'],
        'customerId': !exists(json, 'customerId') ? undefined : json['customerId'],
        'jwt': !exists(json, 'jwt') ? undefined : json['jwt'],
        'loggedIn': !exists(json, 'loggedIn') ? undefined : (new Date(json['loggedIn'])),
        'loggedOut': !exists(json, 'loggedOut') ? undefined : (new Date(json['loggedOut'])),
        'created': !exists(json, 'created') ? undefined : (new Date(json['created'])),
    };
}

export function AuthenticationEntryToJSON(value?: AuthenticationEntry | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'id': value.id,
        'customerId': value.customerId,
        'jwt': value.jwt,
        'loggedIn': value.loggedIn === undefined ? undefined : (value.loggedIn.toISOString()),
        'loggedOut': value.loggedOut === undefined ? undefined : (value.loggedOut.toISOString()),
        'created': value.created === undefined ? undefined : (value.created.toISOString()),
    };
}

