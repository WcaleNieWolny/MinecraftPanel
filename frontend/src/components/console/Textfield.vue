<template>
  <div id="app" class="">
    <div class="wrapper">
      <div class="list" ref="list">
        <p  v-for="item in items" :key="item">
          {{item.text}}
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Item from './Item.vue'
import { getData } from './data'

export default {
  name: 'App',
  data() {
    return {
      item: Item,
      items: getData(1)
    }
  },
  components: {
    Item,
  },
  methods: {
    pushData(string: string){
      this.$data.items.push({
        id: String(this.$data.items.length),
        text: string,
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
  min-height: 85vh;
  max-height: 85vh;
  /* min-height: 10vh;
  max-height: 10vh; */
  line-height: 0.2;
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