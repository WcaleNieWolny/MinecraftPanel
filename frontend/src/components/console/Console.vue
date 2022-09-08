<template>
    <header>
        <div>
            <h1 style="text-align: center;">Console Window</h1>
            <Textfield/>
            <Input text="Send"/>
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
}
</script>

<style scoped>
    header{
        /* justify-content: space-between;
        align-items: center;
        margin-bottom: 20px; */
    }
</style>