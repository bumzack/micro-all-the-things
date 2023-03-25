# Fucking JavaScript 

## compile and start using

``` npx tsc -t es5  -out dist/main.js src/main.ts ../common/dtos.ts  ../common/const.ts --module amd && nodemon serve_static.js```




"build": "rm dist/* &&  yarn parcel build src/index.html && ./blupp.sh",



## uninstall verdaccio

```
npm config set registry https://registry.npmjs.org/
```

