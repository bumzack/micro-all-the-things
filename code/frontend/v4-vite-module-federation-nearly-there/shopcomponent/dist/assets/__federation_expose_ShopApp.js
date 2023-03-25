import { importShared } from './__federation_fn_import.js';

const remotesMap = {
'remoteApp':{url:'http://localhost:7001/assets/remoteEntry.js',format:'esm',from:'vite'},
  'cartApp':{url:'http://localhost:7002/assets/remoteEntry.js',format:'esm',from:'vite'}
};
                const loadJS = async (url, fn) => {
                    const resolvedUrl = typeof url === 'function' ? await url() : url;
                    const script = document.createElement('script');
                    script.type = 'text/javascript';
                    script.onload = fn;
                    script.src = resolvedUrl;
                    document.getElementsByTagName('head')[0].appendChild(script);
                };

                function get(name, remoteFrom) {
                    return __federation_import(name).then(module => () => {
                        if (remoteFrom === 'webpack') {
                            return Object.prototype.toString.call(module).indexOf('Module') > -1 && module.default ? module.default : module
                        }
                        return module
                    })
                }

                const wrapShareModule = remoteFrom => {
                    return {
                        'jquery':{'undefined':{get:()=>get('./__federation_shared_jquery.js', remoteFrom), loaded:1}}
                    }
                };

                async function __federation_import(name) {
                    return import(name);
                }

                async function __federation_method_ensure(remoteId) {
                    const remote = remotesMap[remoteId];
                    if (!remote.inited) {
                        if ('var' === remote.format) {
                            // loading js with script tag
                            return new Promise(resolve => {
                                const callback = () => {
                                    if (!remote.inited) {
                                        remote.lib = window[remoteId];
                                        remote.lib.init(wrapShareModule(remote.from));
                                        remote.inited = true;
                                    }
                                    resolve(remote.lib);
                                };
                                return loadJS(remote.url, callback);
                            });
                        } else if (['esm', 'systemjs'].includes(remote.format)) {
                            // loading js with import(...)
                            return new Promise((resolve, reject) => {
                                const getUrl = typeof remote.url === 'function' ? remote.url : () => Promise.resolve(remote.url);
                                getUrl().then(url => {
                                    import(/* @vite-ignore */ url).then(lib => {
                                        if (!remote.inited) {
                                            const shareScope = wrapShareModule(remote.from);
                                            lib.init(shareScope);
                                            remote.lib = lib;
                                            remote.lib.init(shareScope);
                                            remote.inited = true;
                                        }
                                        resolve(remote.lib);
                                    }).catch(reject);
                                });
                            })
                        }
                    } else {
                        return remote.lib;
                    }
                }

                function __federation_method_wrapDefault(module, need) {
                    if (!module?.default && need) {
                        let obj = Object.create(null);
                        obj.default = module;
                        obj.__esModule = true;
                        return obj;
                    }
                    return module;
                }

                function __federation_method_getRemote(remoteName, componentName) {
                    return __federation_method_ensure(remoteName).then((remote) => remote.get(componentName).then(factory => factory()));
                }

const jQuery = await importShared('jquery');

window.$ = window.jQuery = jQuery;
async function init() {
  console.log("xxxxxxxxxxxx  xxxxx   car app callback");
  let app1 = await __federation_method_getRemote("remoteApp" , "./App1").then(module=>__federation_method_wrapDefault(module, true));
  let cartApp = await __federation_method_getRemote("cartApp" , "./CartApp").then(module=>__federation_method_wrapDefault(module, true));
  const a = app1.default();
  console.log("axxxxxaaa ", a);
  const b = cartApp.default();
  console.log("xxxxx   bbbbb ", b);
}
class ShopComponent extends HTMLElement {
  constructor() {
    super();
    console.log("shop componentn constructor called");
  }
  createShopApp() {
    const root = $("#root");
    console.log("createShopApp");
    root.append("ShopComponent JQUERY hello from jquery slkfjslj lsjf jsldjf ljsfl");
    return `
            <div class="container-fluid">
                <div class="row-lg">
                    left column
                    <div>before</div>
                    <search-component></search-component>
                    <cart-component></cart-component>
                    <div>after</div>
                </div>
                <div class="row-lg">
                    right column
                </div>
            </div>
        `;
  }
  connectedCallback() {
    console.log("before init ShopApp");
    init();
    console.log("after init ShopApp");
    this.innerHTML += this.createShopApp();
  }
}
window.customElements.define("shop-component", ShopComponent);
console.log("hallo shop-component");
let ShopApp = function() {
  return `shop app!`;
};

export { ShopApp, ShopApp as default };
