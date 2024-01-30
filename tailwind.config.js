/** @type {import('tailwindcss').Config} */
const colors = require('tailwindcss/colors')

export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx,vue}",
    ],
    theme: {
        extend: {
          colors: {
            // Dark mode colors
            background: '#121212',
            surface: {
              DEFAULT: '#262626',
              dp1: '#1c1c1c',
              dp2: '#202020',
              dp3: '#242424',
              dp4: '#262626',
              dp6: '#2c2c2c',
              dp8: '#2d2d2d',
              dp12: '#323232',
              dp16: '#353535',
              dp24: '#373737',
            },
            primary: {
              DEFAULT: '#BB86FC',
              variant: '#3700B3'
            },
            secondary: {
              DEFAULT: '#03DAC5',
            },
            error: '#CF6679',
            OnPrimary: '#000000',
            OnSecondary: '#000000',
            OnBackgound: '#FFFFFF',
            OnSurface: '#FFFFFF',
            OnError: '#000000',

            // White mode colors
            wm: {
              background: '#121212',
              surface: {
                DEFAULT: '#e5e5e5'
              },
              primary: {
                DEFAULT: '#6200EE',
                variant: '#3700B3'
              },
              secondary: {
                DEFAULT: '#03DAC5',
                variant: '#018786'
              },
              error: '#B00020',
              OnPrimary: '#FFFFFF',
              OnSecondary: '#000000',
              OnBackgound: '#000000',
              OnSurface: '#000000',
              OnError: '#FFFFFF',
            }
          },
        },
    },
    darkMode: 'class'
}