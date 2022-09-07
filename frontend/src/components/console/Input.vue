<template>
    <div class="main">
        <input v-model="formdata.input" placeholder="minecraft command" style="width: 90%"/>
        <button @click="onClick()" style="width: 3%">{{ text }}</button>
    </div>
</template>

<script lang="ts">
    export default{
        name: 'Input',

        data() {
            return {
                formdata: { input: ''}
            }
        },

        props: {
            text: String,
        },

        methods: {
            onClick() {
                console.log(this.$data.formdata.input)
                this.$data.formdata.input = ""
            },
        }
    }
</script>

<style scoped>
    .main {
        margin-left: 3.5%;
        margin-right: 3.5%;
        padding: 1em;
    }
</style>