<template>
  <div class="shadow-xxl overflow-x-scroll overflow-y-scroll m-auto mt-0 bg-zinc-800 rounded-xl p-2 w-[92%] min-w-[92%] h-[86vh] min-h-[86vh]" ref="list">
    <p  v-for="item in items" :key="item.id">
      <span v-html="item.html" class="text-zinc-300"></span>
    </p>
  </div>
</template>

<script setup lang="ts">
  import { getData } from './data'
  import Convert from 'ansi-to-html'

  const items =  ref(getData(1))
  const converter = new Convert()
  const list = ref<HTMLDivElement>();
  const runtimeConfig = useRuntimeConfig()

  const pushData = (string: string, parseAnsi = true) => {
      items.value.push({
        id: String(items.value.length),
        text: string,
        html: parseAnsi ? converter.toHtml(string) : string
    });
  };


  onMounted(async () => {
    

    const events = new EventSource(`${runtimeConfig.public.apiUrl}/api/console`, { withCredentials: true });

    events.addEventListener('open', function (event) {
      pushData("Connected!")
    });

    events.addEventListener("message", (ev) => {
      pushData(JSON.parse(ev.data))
    });
  })

  onUpdated(() => {
    var container: any = list.value;
    container.scrollTop = container.scrollHeight;
  })

  defineExpose(
    {pushData: pushData}
);
</script>