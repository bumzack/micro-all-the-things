export interface Product {
    id: string,
    article: string,
    articleUnit: string,
    articleName: string,
    articleDescription: string,
    code: string,
    visible: boolean,
    orderable: boolean,
    sourcing: string,
    division: string,
    material: string,
    codeWhg: string,
    supplierName: string,
    defaultSupplier: string,
    otns: Array<string>,
    eans: Array<string>,
    predecessorCodes: Array<string>,
    predecessorEans: Array<string>,
    predecessorOtns: Array<string>,
    module: string,
    moduleGroup: string,
    ownBrand: string,
    superCategories: Array<string>,
    allSuperCategories: Array<string>,
    imagesContainerQualifiers: Array<string>,
    mainImageContainerQualifier: string
}

export interface Image {
    id: string,
    code: string,
    mediaContainerQualifier: string,
    url: string,
    width: string,
    height: string,
    channel: string,
    valid: boolean,
    mediaFormat: string,
    mime: string
}

export interface Article {
    article: Product,
    image: Image,
}

export interface Category {
    id: string,
    code: string,
    name: string,
    superCategories: Array<string>,
    allSuperCategories: Array<string>,
}

export interface SearchResult {
    text: string,
    code: string,
    pageSize: number,
    start: number,
}
