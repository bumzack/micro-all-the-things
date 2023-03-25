import {defineConfig} from 'vite'
import federation from '@originjs/vite-plugin-federation'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        federation({
            name: "remote_app",
            filename: "remoteEntry.js",
            exposes: {
                './App1': './src/main.ts'
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
