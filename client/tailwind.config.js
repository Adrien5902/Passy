/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            animation: {
                'spin-fast': 'spin .3s linear infinite',
            },
            borderWidth: {
                '3': '3px'
            },
            colors: {
                'oxford-blue': {
                    '50': '#f5f7fa',
                    '100': '#ebeff3',
                    '200': '#d2dbe5',
                    '300': '#abbece',
                    '400': '#7d9bb3',
                    '500': '#5d7f9a',
                    '600': '#496580',
                    '700': '#3c5268',
                    '800': '#374a5c',
                    '900': '#2f3d4b',
                    '950': '#1f2732',
                },
            }
        },
    },
    plugins: [],
}