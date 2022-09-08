<template>
    <div class="main">
        <input class="input" v-model="formdata.input" placeholder="minecraft command"/>
        <button class="button" @click="onClick()">{{ text }}</button>
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
        height: 5hv;
        max-width: 95%;
        margin-top: 1px;
        margin: auto;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .input {
        float: left;
        width:  95%;
        height: 2.5vh;
    }
    .button {
        border: none;
        float: right;
        width:  5%;
        height: 3vh;
    }
</style>