<template>
    <div class="bg-zinc-700 h-full flex flex-col">
        <Navbar/>
        <h1 class="text-center text-zinc-200 mt-[0.5vh] mb-[0.5vh] text-base">Console Window</h1>
        <Textfield ref="textfield"/>
        <Input @console-data-send="handleConsoleDataSend" text="Send"/>
    </div>
</template>


<script>
import Input from './Input.vue'
import Textfield from './Textfield.vue'
export default {
    name: 'Console',
    components: {
        Input,
        Textfield,
    },
    methods: {
        handleConsoleDataSend(data){
            this.$refs.textfield.pushData(`> ${data}`, false)
        }
    }
}
</script>