<template>
  <div id="app" class="">
    <div class="wrapper">
      <div class="list" ref="list">
        <p  v-for="item in items" :key="item">
          <span v-html="item.html"></span>
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { getData } from './data'
import Convert from 'ansi-to-html'

export default {
  name: 'App',
  data() {
    return {
      items: getData(1)
    }
  },
  methods: {
    pushData(string: string, parseAnsi = true){
      this.$data.items.push({
        id: String(this.$data.items.length),
        text: string,
        html: parseAnsi ? new Convert().toHtml(string) : string
      });
    },
  },
  mounted() {
    let socket = new WebSocket("ws://127.0.0.1:3001")

    let pushData = this.pushData;

    socket.addEventListener('open', function (event) {
      pushData("Connected!")
    });

    socket.addEventListener('message', function (event) {
        pushData(event.data)
    });

    console.log(`the component is now mounted.`)
  },
  updated() {
    var container: any = this.$refs.list;
    container.scrollTop = container.scrollHeight;
  },
}
</script>

<style>
.this_aaa {
  display: block;
  width: 95%;
  color: #2c3e50;
  margin: 0 auto;
  margin-left: 0 auto;
  margin-right: 0 auto;
  min-height: 85vh;
  max-height: 85vh;
  padding: 1em;
}
.list {
  display: block;
  width: 95%;
  border: 2px solid red;
  min-height: 92vh;
  max-height: 92vh;
  /* min-height: 10vh;
  max-height: 10vh; */
  line-height: 1;
  font-size: 1.115rem;
  margin: 0 auto;
  margin-left: 0 auto;
  margin-right: 0 auto;
  border-radius: 3px;
  overflow:scroll;
  overflow-x: scroll;
  overflow-y: none;
}
</style>