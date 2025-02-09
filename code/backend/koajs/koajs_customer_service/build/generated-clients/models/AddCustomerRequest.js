"use strict";
/* tslint:disable */
/* eslint-disable */
/**
 * Customer Service
 * Customer Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.AddCustomerRequestToJSON = exports.AddCustomerRequestFromJSONTyped = exports.AddCustomerRequestFromJSON = exports.instanceOfAddCustomerRequest = void 0;
const runtime_1 = require("../runtime");
/**
 * Check if a given object implements the AddCustomerRequest interface.
 */
function instanceOfAddCustomerRequest(value) {
    let isInstance = true;
    return isInstance;
}
exports.instanceOfAddCustomerRequest = instanceOfAddCustomerRequest;
function AddCustomerRequestFromJSON(json) {
    return AddCustomerRequestFromJSONTyped(json, false);
}
exports.AddCustomerRequestFromJSON = AddCustomerRequestFromJSON;
function AddCustomerRequestFromJSONTyped(json, ignoreDiscriminator) {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        'firstName': !(0, runtime_1.exists)(json, 'firstName') ? undefined : json['firstName'],
        'lastName': !(0, runtime_1.exists)(json, 'lastName') ? undefined : json['lastName'],
        'email': !(0, runtime_1.exists)(json, 'email') ? undefined : json['email'],
        'password': !(0, runtime_1.exists)(json, 'password') ? undefined : json['password'],
    };
}
exports.AddCustomerRequestFromJSONTyped = AddCustomerRequestFromJSONTyped;
function AddCustomerRequestToJSON(value) {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        'firstName': value.firstName,
        'lastName': value.lastName,
        'email': value.email,
        'password': value.password,
    };
}
exports.AddCustomerRequestToJSON = AddCustomerRequestToJSON;
//# sourceMappingURL=AddCustomerRequest.js.map