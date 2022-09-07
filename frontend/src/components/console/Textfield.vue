<template>
  <div id="app">
    <div class="wrapper">
      <div class="list">
        <p  v-for="item in items" :key="item">
          {{item}}
        </p>
      </div>
    </div>
  </div>
</template>

<script>
import Item from './Item.vue'
import { getData } from './data'

export default {
  name: 'App',
  data() {
    return {
      item: Item,
      items: getData(100)
    }
  },
  components: {
    Item,
  }
}
</script>

<style>
#app {
  text-align: left;
  color: #2c3e50;
  margin-top: 1em;
  margin-left: 3.5%;
  margin-right: 3.5%;
  max-height: 85vh;
  padding: 1em;
}
.list {
  border: 2px solid red;
  max-height: 85vh;
  border-radius: 3px;
  overflow:scroll;
  overflow-x: scroll;
  overflow-y: scroll;
}
</style>