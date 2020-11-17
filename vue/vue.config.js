module.exports = {
  "transpileDependencies": [
    "vuetify"
  ],
  pages: {
    index: {
      entry: 'src/main.js',
      template: 'public/index.html',
      filename: 'index.html',
    },
    login: {
      entry: 'src/main.js',
      template: 'public/login.html',
      filename: 'login.html',
    }
  }
}
