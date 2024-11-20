import { defineConfig } from 'vite';

export default defineConfig({
    root: './ui',
    base: '/rust-tinybasic-interpreter/',
    build: {
        outDir: '../docs'
    },
    server: {
        headers: {
            'Cross-Origin-Opener-Policy': 'same-origin',
            'Cross-Origin-Embedder-Policy': 'require-corp'
        }
    }
});
