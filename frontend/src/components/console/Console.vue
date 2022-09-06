<template>
    <header>
        <h1>Console Window</h1>
        <Input text="Add Tasks"/>
    </header>  
</template>


<script>
import Input from './Input.vue'

export default {
    name: 'Console',

    components: {
        Input,
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