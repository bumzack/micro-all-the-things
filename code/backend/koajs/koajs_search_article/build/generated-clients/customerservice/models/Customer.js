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
exports.CustomerToJSON = exports.CustomerFromJSONTyped = exports.CustomerFromJSON = exports.instanceOfCustomer = void 0;
const runtime_1 = require("../runtime");
/**
 * Check if a given object implements the Customer interface.
 */
function instanceOfCustomer(value) {
    let isInstance = true;
    return isInstance;
}
exports.instanceOfCustomer = instanceOfCustomer;
function CustomerFromJSON(json) {
    return CustomerFromJSONTyped(json, false);
}
exports.CustomerFromJSON = CustomerFromJSON;
function CustomerFromJSONTyped(json, ignoreDiscriminator) {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        'id': !(0, runtime_1.exists)(json, 'id') ? undefined : json['id'],
        'firstName': !(0, runtime_1.exists)(json, 'firstName') ? undefined : json['firstName'],
        'lastName': !(0, runtime_1.exists)(json, 'lastName') ? undefined : json['lastName'],
        'email': !(0, runtime_1.exists)(json, 'email') ? undefined : json['email'],
        'password': !(0, runtime_1.exists)(json, 'password') ? undefined : json['password'],
        'created': !(0, runtime_1.exists)(json, 'created') ? undefined : (new Date(json['created'])),
    };
}
exports.CustomerFromJSONTyped = CustomerFromJSONTyped;
function CustomerToJSON(value) {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        'id': value.id,
        'firstName': value.firstName,
        'lastName': value.lastName,
        'email': value.email,
        'password': value.password,
        'created': value.created === undefined ? undefined : (value.created.toISOString()),
    };
}
exports.CustomerToJSON = CustomerToJSON;
//# sourceMappingURL=Customer.js.map