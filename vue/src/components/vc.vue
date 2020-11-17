<template>
  <v-dialog v-model="dialog">
    <template v-slot:activator="{ on, attrs }">
      <v-btn v-bind="attrs" v-on="on">Issue VC</v-btn>
    </template>
    <v-card>
      <v-card-title class="headline">Issue VC {{ keyName }}</v-card-title>
      <v-card-text>
        <v-form>
          <v-file-input v-model="file" label="File" name="file" required></v-file-input><br>
          <v-btn @click="submit">Issue VC</v-btn>
        </v-form>
        <br/>
        <pre v-if="vc"><code>{{ JSON.stringify(vc, null, 4) }}</code></pre>
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
  name: 'vc',
  props: {
    keyName: {
      type: String,
      required: true,
    },
  },
  data: () => ({
    dialog: false,
    file: null,
    vc: null
  }),
  methods: {
    submit () {
      const form = new FormData();
      form.append('key', this.keyName);
      form.append('file', this.file);
      axios.post('/vc', form).then(response => (this.vc = response.data)) }
  }
}
</script>
