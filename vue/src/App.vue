<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <h1>Keylink</h1>
    </v-app-bar>

    <v-main>
      <keys v-bind:keys="keys" />
    </v-main>
  </v-app>
</template>

<script>
import keys from './components/keys.vue'

export default {
  name: 'App',
  props: {
    keys: Array
  },
  components: {
    keys
  }
}
</script>
