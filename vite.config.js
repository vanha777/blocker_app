import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: '127.0.0.1',
    port: 8080,
    strictPort: true,
    historyApiFallback: true,
    proxy: 'http://localhost:3030',
    cors:true
  },
})
