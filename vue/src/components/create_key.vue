<template>
  <v-dialog v-model="dialog">
    <template v-slot:activator="{ on, attrs }">
      <v-btn v-bind="attrs" v-on="on">New Key</v-btn>
    </template>
    <v-card>
      <v-card-title class="headline">New Key</v-card-title>
      <v-card-text>
        <v-form>
          <v-text-field v-model="name" label="Name" name="name" required></v-text-field><br>
          <v-btn @click="submit">Create Key</v-btn>
        </v-form>
      </v-card-text>
      <v-card-actions>
        <v-btn text @click="dialog = false">Close</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
const axios = require('axios').default;

export default {
  name: 'createKey',
  props: {
    keys: Array
  },
  data: () => ({
    dialog: false,
    name: null,
  }),
  methods: {
    submit () {
      const form = new URLSearchParams();
      form.append('name', this.name);
      axios.post('/keys', form).then(this.keys.append(this.name),
                                     this.dialog = false)
    }
  }
}
</script>
