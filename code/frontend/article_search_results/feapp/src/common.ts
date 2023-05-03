export interface SearchIndexDoc {
    id: string,
    tconst: string,
    titles?: Array<string>,
    actors?: Array<string>,
    directors?: Array<string>,
    writers?: Array<string>,
    runtime_minutes?: number,
    adult?: boolean,
    genres?: Array<string>,
    characters?: Array<string>,
    title_type?: Array<string>,
    year?: number,
    primary_title?: string,
    original_title?: string,
}


export interface ArticleSearchResult {
    article: SearchIndexDoc,
    price: number,
    customer_price?: number,
}

export interface SearchArticleResponse {
    articles?: Array<ArticleSearchResult>,
    facets?: IndexDocFacetDistribution,
}

export interface IndexDocFacetDistribution {
    actors?: Map<String, Map<String, number>>,
    directors?: Map<String, Map<String, number>>,
    genres?: Map<String, Map<String, number>>,
    titles?: Map<String, Map<String, number>>,
    characters?: Map<String, Map<String, number>>,
}

export interface SearchCustomer {
    customer_id?: number,
    jwt?: string,
}

export interface SearchArticleRequest {
    q: string,
    offset: number,
    limit: number,
    customer: SearchCustomer,
}


