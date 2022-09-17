<template>
  <div id="app" class="">
    <div class="wrapper">
      <div class="w-[95%] border-2 border-cyan-700 border-solid max-h-[93vh] min-h-[93vh] rounded-lg leading-none text-lg m-auto overflow-x-scroll overflow-y-hidden bg-zinc-800" ref="list">
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