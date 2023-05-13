"use strict";
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
Object.defineProperty(exports, "__esModule", { value: true });
exports.AuthenticationEntryToJSON = exports.AuthenticationEntryFromJSONTyped = exports.AuthenticationEntryFromJSON = exports.instanceOfAuthenticationEntry = void 0;
const runtime_1 = require("../runtime");
/**
 * Check if a given object implements the AuthenticationEntry interface.
 */
function instanceOfAuthenticationEntry(value) {
    let isInstance = true;
    return isInstance;
}
exports.instanceOfAuthenticationEntry = instanceOfAuthenticationEntry;
function AuthenticationEntryFromJSON(json) {
    return AuthenticationEntryFromJSONTyped(json, false);
}
exports.AuthenticationEntryFromJSON = AuthenticationEntryFromJSON;
function AuthenticationEntryFromJSONTyped(json, ignoreDiscriminator) {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        'id': !(0, runtime_1.exists)(json, 'id') ? undefined : json['id'],
        'customerId': !(0, runtime_1.exists)(json, 'customerId') ? undefined : json['customerId'],
        'jwt': !(0, runtime_1.exists)(json, 'jwt') ? undefined : json['jwt'],
        'loggedIn': !(0, runtime_1.exists)(json, 'loggedIn') ? undefined : (new Date(json['loggedIn'])),
        'loggedOut': !(0, runtime_1.exists)(json, 'loggedOut') ? undefined : (new Date(json['loggedOut'])),
        'created': !(0, runtime_1.exists)(json, 'created') ? undefined : (new Date(json['created'])),
    };
}
exports.AuthenticationEntryFromJSONTyped = AuthenticationEntryFromJSONTyped;
function AuthenticationEntryToJSON(value) {
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
exports.AuthenticationEntryToJSON = AuthenticationEntryToJSON;
//# sourceMappingURL=AuthenticationEntry.js.map