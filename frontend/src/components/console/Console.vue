<template>
    <header>
        <h1>Console Window</h1>
        <Textfield/>
        <Input text="Add Tasks"/>
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
        display: inline;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
    }
</style>