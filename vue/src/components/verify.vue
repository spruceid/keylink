<template>
  <v-dialog v-model="dialog">
    <template v-slot:activator="{ on, attrs }">
      <v-btn v-bind="attrs" v-on="on">Verify</v-btn>
    </template>
    <v-card>
      <v-card-title class="headline">Verify Signature {{ keyName }}</v-card-title>
      <v-card-text>
        <v-form>
          <v-textarea v-model="doc" label="Signed Text" name="doc" required></v-textarea><br>
          <v-textarea v-model="sig" label="Signature" name="sig" required></v-textarea><br>
          <v-btn @click="submit">Verify</v-btn>
        </v-form>
        <br/>
        <v-btn depressed color="success" v-if="check">Valid</v-btn>
        <v-btn depressed color="error" v-else-if="check === false">Invalid</v-btn>
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
  name: 'verify',
  props: {
    keyName: {
      type: String,
      required: true,
    },
  },
  data: () => ({
    dialog: false,
    doc: null,
    sig: null,
    check: null
  }),
  methods: {
    submit () {
      const form = new URLSearchParams();
      form.append('sig', this.sig);
      form.append('key', this.keyName);
      form.append('doc', this.doc);
      axios.post('/verify', form).then(response => (this.check = response.data)) }
  }
}
</script>

<style scoped>
.check {
    border-radius: .4rem;
    text-transform: uppercase;
    text-align: center;
    font-weight: 600;
    display: block;
    padding: 1rem;
    color: #fff;
}
.valid {
    background-color: #7CCA4D;
}
.invalid {
    background-color: #CA5C4D;
}
</style>
