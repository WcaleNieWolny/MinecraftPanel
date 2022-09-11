<template>
    <header>
        <div>
            <h1 style="text-align: center;">Console Window</h1>
            <Textfield ref="textfield"/>
            <Input @console-data-send="handleConsoleDataSend" text="Send"/>
        </div>
    </header>  
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
            this.$refs.textfield.pushData(`> ${data}`)
        }
    }
}
</script>