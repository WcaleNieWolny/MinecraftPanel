<template>
  <div id="app" class="">
    <div>
      <div class="w-[95%] max-h-[87.5vh] min-h-[87.5vh] rounded-lg shadow-xxl leading-none text-lg m-auto  mb-auto overflow-x-scroll overflow-y-scroll bg-zinc-800" ref="list">
        <p  v-for="item in items" :key="item.id">
          <span v-html="item.html" class="text-zinc-300"></span>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { getData } from './data'
  import Convert from 'ansi-to-html'
  import Input from './Input.vue'

  const items =  ref(getData(1))
  const converter = new Convert()
  const list = ref<HTMLDivElement>();

  const pushData = (string: string, parseAnsi = true) => {
    console.log(string)
      items.value.push({
        id: String(items.value.length),
        text: string,
        html: parseAnsi ? converter.toHtml(string) : string
    });
  };


  onMounted(async () => {
    const apiUrl = useApiUrl()

    let http_response = await fetch(`${apiUrl.value}/auth/request_console`, {    
        method: 'GET',
        cache: 'no-cache',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
    })
    let json = await http_response.json();

    console.log(json)

    let socket = new WebSocket("ws://127.0.0.1:3001")

    socket.addEventListener('open', function (event) {
      socket.send(json.hash)
      pushData("Connected!")
    });

    socket.addEventListener('message', function (event) {
        pushData(event.data)
    });

    console.log(`the component is now mounted.`)
  })

  const updated = () => {
    var container: any = list;
    container.scrollTop = container.scrollHeight;
  }

  defineExpose(
    {pushData: pushData}
);
</script>