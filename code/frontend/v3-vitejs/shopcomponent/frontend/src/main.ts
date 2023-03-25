/// <reference path ="../node_modules/@types/jquery/JQuery.d.ts"/>


// src/main.ts:5:9 - error TS2669: Augmentations for the global scope can only be directly nested in external modules or ambient module declarations.
export {};

// // https://isotropic.co/how-to-fix-the-property-does-not-exist-on-type-window-error-in-typescript/
declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery:any;
    }
}

window.$ = window.jQuery = jQuery;

console.log("main");