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
exports.TextApiResponse = exports.BlobApiResponse = exports.VoidApiResponse = exports.JSONApiResponse = exports.canConsumeForm = exports.mapValues = exports.querystring = exports.exists = exports.COLLECTION_FORMATS = exports.RequiredError = exports.FetchError = exports.ResponseError = exports.BaseAPI = exports.DefaultConfig = exports.Configuration = exports.BASE_PATH = void 0;
exports.BASE_PATH = "http://localhost:8320".replace(/\/+$/, "");

class Configuration {
    constructor(configuration = {}) {
        this.configuration = configuration;
    }

    set config(configuration) {
        this.configuration = configuration;
    }

    get basePath() {
        return this.configuration.basePath != null ? this.configuration.basePath : exports.BASE_PATH;
    }

    get fetchApi() {
        return this.configuration.fetchApi;
    }

    get middleware() {
        return this.configuration.middleware || [];
    }

    get queryParamsStringify() {
        return this.configuration.queryParamsStringify || querystring;
    }

    get username() {
        return this.configuration.username;
    }

    get password() {
        return this.configuration.password;
    }

    get apiKey() {
        const apiKey = this.configuration.apiKey;
        if (apiKey) {
            return typeof apiKey === 'function' ? apiKey : () => apiKey;
        }
        return undefined;
    }

    get accessToken() {
        const accessToken = this.configuration.accessToken;
        if (accessToken) {
            return typeof accessToken === 'function' ? accessToken : async () => accessToken;
        }
        return undefined;
    }

    get headers() {
        return this.configuration.headers;
    }

    get credentials() {
        return this.configuration.credentials;
    }
}

exports.Configuration = Configuration;
exports.DefaultConfig = new Configuration();

/**
 * This is the base class for all generated API classes.
 */
class BaseAPI {
    constructor(configuration = exports.DefaultConfig) {
        this.configuration = configuration;
        this.fetchApi = async (url, init) => {
            let fetchParams = {url, init};
            for (const middleware of this.middleware) {
                if (middleware.pre) {
                    fetchParams = await middleware.pre({
                        fetch: this.fetchApi,
                        ...fetchParams,
                    }) || fetchParams;
                }
            }
            let response = undefined;
            try {
                response = await (this.configuration.fetchApi || fetch)(fetchParams.url, fetchParams.init);
            } catch (e) {
                for (const middleware of this.middleware) {
                    if (middleware.onError) {
                        response = await middleware.onError({
                            fetch: this.fetchApi,
                            url: fetchParams.url,
                            init: fetchParams.init,
                            error: e,
                            response: response ? response.clone() : undefined,
                        }) || response;
                    }
                }
                if (response === undefined) {
                    if (e instanceof Error) {
                        throw new FetchError(e, 'The request failed and the interceptors did not return an alternative response');
                    } else {
                        throw e;
                    }
                }
            }
            for (const middleware of this.middleware) {
                if (middleware.post) {
                    response = await middleware.post({
                        fetch: this.fetchApi,
                        url: fetchParams.url,
                        init: fetchParams.init,
                        response: response.clone(),
                    }) || response;
                }
            }
            return response;
        };
        this.middleware = configuration.middleware;
    }

    withMiddleware(...middlewares) {
        const next = this.clone();
        next.middleware = next.middleware.concat(...middlewares);
        return next;
    }

    withPreMiddleware(...preMiddlewares) {
        const middlewares = preMiddlewares.map((pre) => ({pre}));
        return this.withMiddleware(...middlewares);
    }

    withPostMiddleware(...postMiddlewares) {
        const middlewares = postMiddlewares.map((post) => ({post}));
        return this.withMiddleware(...middlewares);
    }

    async request(context, initOverrides) {
        const {url, init} = await this.createFetchParams(context, initOverrides);
        const response = await this.fetchApi(url, init);
        if (response && (response.status >= 200 && response.status < 300)) {
            return response;
        }
        throw new ResponseError(response, 'Response returned an error code');
    }

    async createFetchParams(context, initOverrides) {
        let url = this.configuration.basePath + context.path;
        if (context.query !== undefined && Object.keys(context.query).length !== 0) {
            // only add the querystring to the URL if there are query parameters.
            // this is done to avoid urls ending with a "?" character which buggy webservers
            // do not handle correctly sometimes.
            url += '?' + this.configuration.queryParamsStringify(context.query);
        }
        const headers = Object.assign({}, this.configuration.headers, context.headers);
        Object.keys(headers).forEach(key => headers[key] === undefined ? delete headers[key] : {});
        const initOverrideFn = typeof initOverrides === "function"
            ? initOverrides
            : async () => initOverrides;
        const initParams = {
            method: context.method,
            headers,
            body: context.body,
            credentials: this.configuration.credentials,
        };
        const overriddenInit = {
            ...initParams,
            ...(await initOverrideFn({
                init: initParams,
                context,
            }))
        };
        const init = {
            ...overriddenInit,
            body: isFormData(overriddenInit.body) ||
            overriddenInit.body instanceof URLSearchParams ||
            isBlob(overriddenInit.body)
                ? overriddenInit.body
                : JSON.stringify(overriddenInit.body),
        };
        return {url, init};
    }

    /**
     * Create a shallow clone of `this` by constructing a new instance
     * and then shallow cloning data members.
     */
    clone() {
        const constructor = this.constructor;
        const next = new constructor(this.configuration);
        next.middleware = this.middleware.slice();
        return next;
    }
}

exports.BaseAPI = BaseAPI;
;

function isBlob(value) {
    return typeof Blob !== 'undefined' && value instanceof Blob;
}

function isFormData(value) {
    return typeof FormData !== "undefined" && value instanceof FormData;
}

class ResponseError extends Error {
    constructor(response, msg) {
        super(msg);
        this.response = response;
        this.name = "ResponseError";
    }
}

exports.ResponseError = ResponseError;

class FetchError extends Error {
    constructor(cause, msg) {
        super(msg);
        this.cause = cause;
        this.name = "FetchError";
    }
}

exports.FetchError = FetchError;

class RequiredError extends Error {
    constructor(field, msg) {
        super(msg);
        this.field = field;
        this.name = "RequiredError";
    }
}

exports.RequiredError = RequiredError;
exports.COLLECTION_FORMATS = {
    csv: ",",
    ssv: " ",
    tsv: "\t",
    pipes: "|",
};

function exists(json, key) {
    const value = json[key];
    return value !== null && value !== undefined;
}

exports.exists = exists;

function querystring(params, prefix = '') {
    return Object.keys(params)
        .map(key => querystringSingleKey(key, params[key], prefix))
        .filter(part => part.length > 0)
        .join('&');
}

exports.querystring = querystring;

function querystringSingleKey(key, value, keyPrefix = '') {
    const fullKey = keyPrefix + (keyPrefix.length ? `[${key}]` : key);
    if (value instanceof Array) {
        const multiValue = value.map(singleValue => encodeURIComponent(String(singleValue)))
            .join(`&${encodeURIComponent(fullKey)}=`);
        return `${encodeURIComponent(fullKey)}=${multiValue}`;
    }
    if (value instanceof Set) {
        const valueAsArray = Array.from(value);
        return querystringSingleKey(key, valueAsArray, keyPrefix);
    }
    if (value instanceof Date) {
        return `${encodeURIComponent(fullKey)}=${encodeURIComponent(value.toISOString())}`;
    }
    if (value instanceof Object) {
        return querystring(value, fullKey);
    }
    return `${encodeURIComponent(fullKey)}=${encodeURIComponent(String(value))}`;
}

function mapValues(data, fn) {
    return Object.keys(data).reduce((acc, key) => ({...acc, [key]: fn(data[key])}), {});
}

exports.mapValues = mapValues;

function canConsumeForm(consumes) {
    for (const consume of consumes) {
        if ('multipart/form-data' === consume.contentType) {
            return true;
        }
    }
    return false;
}

exports.canConsumeForm = canConsumeForm;

class JSONApiResponse {
    constructor(raw, transformer = (jsonValue) => jsonValue) {
        this.raw = raw;
        this.transformer = transformer;
    }

    async value() {
        return this.transformer(await this.raw.json());
    }
}

exports.JSONApiResponse = JSONApiResponse;

class VoidApiResponse {
    constructor(raw) {
        this.raw = raw;
    }

    async value() {
        return undefined;
    }
}

exports.VoidApiResponse = VoidApiResponse;

class BlobApiResponse {
    constructor(raw) {
        this.raw = raw;
    }

    async value() {
        return await this.raw.blob();
    }
    ;
}

exports.BlobApiResponse = BlobApiResponse;

class TextApiResponse {
    constructor(raw) {
        this.raw = raw;
    }

    async value() {
        return await this.raw.text();
    }
    ;
}

exports.TextApiResponse = TextApiResponse;
//# sourceMappingURL=runtime.js.map