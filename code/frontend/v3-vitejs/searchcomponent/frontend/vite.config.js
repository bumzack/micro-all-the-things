import { defineConfig } from 'vite'

export default defineConfig({
    build: {
        rollupOptions: {
            output: {
                entryFileNames: `assets/[name].js`,
                chunkFileNames: `assets/[name].js`,
                assetFileNames: `assets/[name].[ext]`
            }
        },
        minify: false,
        minifySyntax: false
    },
    esbuild: {
        minify: false,
        minifySyntax: false
    },
})
