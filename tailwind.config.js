/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        'htpink': {
          100: '#FFD0D0',
          200: '#FF9EAA',
          300: '#EF2F88',
          400: '#8843F2',
          500: '#C400C6',
          600: '#57007E',
          700: '#090057',
        },
        'htblue': {
          100: '#C1ECE4',
          200: '#3AA6B9',
          300: '#6EACDA',
          400: '#03346E',
          500: '#021526',
        },
        'ht': {
          100: '#F9D371',
          200: '#E2E2B6',
          300: '#FFA069',
          400: '#F47340',
        },
      },
    },
  },
  plugins: [],
}

