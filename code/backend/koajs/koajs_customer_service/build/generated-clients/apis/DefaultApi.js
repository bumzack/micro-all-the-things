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
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.DefaultApi = void 0;
const runtime = __importStar(require("../runtime"));
const models_1 = require("../models");
/**
 *
 */
class DefaultApi extends runtime.BaseAPI {
    /**
     */
    async addCustomerRaw(requestParameters, initOverrides) {
        const queryParameters = {};
        const headerParameters = {};
        headerParameters['Content-Type'] = 'application/json';
        const response = await this.request({
            path: `/api/v1/customer`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: (0, models_1.AddCustomerRequestToJSON)(requestParameters.addCustomerRequest),
        }, initOverrides);
        return new runtime.JSONApiResponse(response, (jsonValue) => (0, models_1.CustomerFromJSON)(jsonValue));
    }
    /**
     */
    async addCustomer(requestParameters = {}, initOverrides) {
        const response = await this.addCustomerRaw(requestParameters, initOverrides);
        return await response.value();
    }
    /**
     */
    async getCustomerRaw(requestParameters, initOverrides) {
        if (requestParameters.email === null || requestParameters.email === undefined) {
            throw new runtime.RequiredError('email', 'Required parameter requestParameters.email was null or undefined when calling getCustomer.');
        }
        const queryParameters = {};
        const headerParameters = {};
        const response = await this.request({
            path: `/api/v1/customer/{email}`.replace(`{${"email"}}`, encodeURIComponent(String(requestParameters.email))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);
        return new runtime.JSONApiResponse(response, (jsonValue) => (0, models_1.CustomerFromJSON)(jsonValue));
    }
    /**
     */
    async getCustomer(requestParameters, initOverrides) {
        const response = await this.getCustomerRaw(requestParameters, initOverrides);
        return await response.value();
    }
}
exports.DefaultApi = DefaultApi;
//# sourceMappingURL=DefaultApi.js.map