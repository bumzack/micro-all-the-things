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
exports.LogOutRequestToJSON = exports.LogOutRequestFromJSONTyped = exports.LogOutRequestFromJSON = exports.instanceOfLogOutRequest = void 0;
const runtime_1 = require("../runtime");
/**
 * Check if a given object implements the LogOutRequest interface.
 */
function instanceOfLogOutRequest(value) {
    let isInstance = true;
    return isInstance;
}
exports.instanceOfLogOutRequest = instanceOfLogOutRequest;
function LogOutRequestFromJSON(json) {
    return LogOutRequestFromJSONTyped(json, false);
}
exports.LogOutRequestFromJSON = LogOutRequestFromJSON;
function LogOutRequestFromJSONTyped(json, ignoreDiscriminator) {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        'customerId': !(0, runtime_1.exists)(json, 'customerId') ? undefined : json['customerId'],
    };
}
exports.LogOutRequestFromJSONTyped = LogOutRequestFromJSONTyped;
function LogOutRequestToJSON(value) {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        'customerId': value.customerId,
    };
}
exports.LogOutRequestToJSON = LogOutRequestToJSON;
//# sourceMappingURL=LogOutRequest.js.map