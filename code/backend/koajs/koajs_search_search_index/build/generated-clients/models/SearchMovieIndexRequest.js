"use strict";
/* tslint:disable */
/* eslint-disable */
/**
 * Search Index Index Service
 * Search Index Index Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */
Object.defineProperty(exports, "__esModule", {value: true});
exports.SearchMovieIndexRequestToJSON = exports.SearchMovieIndexRequestFromJSONTyped = exports.SearchMovieIndexRequestFromJSON = exports.instanceOfSearchMovieIndexRequest = void 0;
const runtime_1 = require("../runtime");

/**
 * Check if a given object implements the SearchMovieIndexRequest interface.
 */
function instanceOfSearchMovieIndexRequest(value) {
    let isInstance = true;
    return isInstance;
}

exports.instanceOfSearchMovieIndexRequest = instanceOfSearchMovieIndexRequest;

function SearchMovieIndexRequestFromJSON(json) {
    return SearchMovieIndexRequestFromJSONTyped(json, false);
}

exports.SearchMovieIndexRequestFromJSON = SearchMovieIndexRequestFromJSON;

function SearchMovieIndexRequestFromJSONTyped(json, ignoreDiscriminator) {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        'q': !(0, runtime_1.exists)(json, 'q') ? undefined : json['q'],
        'offset': !(0, runtime_1.exists)(json, 'offset') ? undefined : json['offset'],
        'limit': !(0, runtime_1.exists)(json, 'limit') ? undefined : json['limit'],
    };
}

exports.SearchMovieIndexRequestFromJSONTyped = SearchMovieIndexRequestFromJSONTyped;

function SearchMovieIndexRequestToJSON(value) {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        'q': value.q,
        'offset': value.offset,
        'limit': value.limit,
    };
}

exports.SearchMovieIndexRequestToJSON = SearchMovieIndexRequestToJSON;
//# sourceMappingURL=SearchMovieIndexRequest.js.map