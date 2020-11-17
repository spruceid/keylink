import Vue from 'vue'
import App from './App.vue'
import Login from './Login.vue'
import vuetify from './plugins/vuetify';

const axios = require('axios').default;
axios.defaults.withCredentials = true

Vue.config.productionTip = false

new Vue({
  render(h){ return h(App, {props: {keys: this.keys}})},

  data () {
    return {
      keys: null,
    }
  },

  vuetify,

  mounted () {
    axios
      .get('/keys')
      .then(response => (this.keys = response.data))
  }
}).$mount('#app')

new Vue({
  render: h => h(Login),
  vuetify,
}).$mount('#login')
