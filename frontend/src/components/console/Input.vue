<template>
    <input v-model="formdata.input" placeholder="minecraft command" />
    <button @click="onClick()">{{ text }}</button>
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