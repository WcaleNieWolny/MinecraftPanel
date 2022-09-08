<template>
  <div id="app" class="">
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
      items: getData(1)
    }
  },
  components: {
    Item,
  }
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
  margin: 0 auto;
  margin-left: 0 auto;
  margin-right: 0 auto;
  border-radius: 3px;
  overflow:scroll;
  overflow-x: scroll;
  overflow-y: none;
}
</style>