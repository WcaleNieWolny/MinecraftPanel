<template>
    <div class="bg-zinc-700 h-full">
        <h1 class="text-center">Console Window</h1>
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
            console.log(this.$refs.textfield.pushData)
            this.$refs.textfield.pushData(`> ${data}`, false)
        }
    }
}
</script>