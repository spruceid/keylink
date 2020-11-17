<template>
  <div class="keys">
    <h1>My Keys</h1>
    <v-container>
      <v-row v-for="key in keys" v-bind:key="key" class="key">
        <v-col><b>{{ key }}</b></v-col>
        <v-col>
          <v-container>
            <v-row>
              <v-col>
                <sign :key-name="key"/>
              </v-col>
              <v-col>
                <vc :key-name="key"/>
              </v-col>
            </v-row>
            <v-row>
              <v-col>
                <verify :key-name="key"/>
              </v-col>
            </v-row>
          </v-container>
        </v-col>
      </v-row>
    </v-container>
    <v-container>
        <createKey :keys="keys"/>
    </v-container>
  </div>
</template>

<script>
import sign from './sign.vue'
import verify from './verify.vue'
import vc from './vc.vue'
import createKey from './create_key.vue'

export default {
  name: 'keys',
  props: {
    keys: Array
  },
  components: {
    sign,
    verify,
    vc,
    createKey
  }
}
</script>

<style scoped>
.key {
    border-style: solid;
    border-width: 1px;
    margin-bottom: 1rem;
}
.blob {
    word-break: break-all;
    overflow-wrap: anywhere;
    word-wrap: break-word;
}
/* textarea { */
/*     white-space: pre; */
/*     overflow-wrap: normal; */
/*     overflow-x: scroll; */
/* } */
</style>
