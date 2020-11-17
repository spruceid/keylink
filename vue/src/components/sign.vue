<template>
  <v-dialog v-model="dialog">
    <template v-slot:activator="{ on, attrs }">
      <v-btn v-bind="attrs" v-on="on">Sign</v-btn>
    </template>
    <v-card>
      <v-card-title class="headline">Sign Document {{ keyName }}</v-card-title>
      <v-card-text>
        <v-form>
          <v-textarea v-model="doc" label="Text to sign" name="doc" required></v-textarea><br>
          <v-btn @click="submit">Sign Text</v-btn>
        </v-form>
        <br/>
        <div v-if="signedDoc">
          <code>{{ signedDoc }}</code>
          <br/>
          <v-btn class="btn" @click="copyText">Copy</v-btn>
        </div>
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
  name: 'sign',
  props: {
    keyName: {
      type: String,
      required: true,
    },
  },
  data: () => ({
    dialog: false,
    doc: null,
    signedDoc: ""
  }),
  methods: {
    submit () {
      const form = new URLSearchParams();
      form.append('key', this.keyName);
      form.append('doc', this.doc);
      axios.post('/sign', form).then(response => (this.signedDoc = response.data))
    },
    copyText() {
      navigator.clipboard.writeText(this.signedDoc);
    }
  }
}
</script>
