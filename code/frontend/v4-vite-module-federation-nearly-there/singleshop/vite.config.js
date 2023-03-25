import {defineConfig} from 'vite'
import federation from '@originjs/vite-plugin-federation'

export default defineConfig({
    plugins: [
        federation({
            name: 'singleshop',
            remotes: {
                ShopApp: 'ShopApp@http://localhost:7000/assets/remoteEntry.js'
            },
            shared: ['jquery']
        })
    ],
    build: {
        modulePreload: false,
        target: 'esnext',
        minify: false,
        cssCodeSplit: false,
        rollupOptions: {
            output: {
                entryFileNames: `assets/[name].js`,
                chunkFileNames: `assets/[name].js`,
                assetFileNames: `assets/[name].[ext]`
            }
        },
        plugins: [
            {
                name: 'no-treeshake',
                transform(_, id) {
                    return {moduleSideEffects: 'no-treeshake'}
                }
            }
        ]
    },
    shared: ['jquery']
})
