<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <h1>Keylink</h1>
    </v-app-bar>

    <v-main>
      <v-container fill-height>
        <v-row align="center" justify="center">
          <v-col class="text-center">
            <a href="/login"><v-btn color="primary">Login</v-btn></a>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
export default {
  name: 'Login',
}
</script>
