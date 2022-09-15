<template>
    <div class="main">
        <input class="input" v-model="formdata.input" placeholder="minecraft command"/>
        <button class="button" @click="onClick()"><svg height="34px" width="34px" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="m23.968 0-23.968 10.286h13.68v13.714z"/></svg></button>
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
            async onClick() {
                var data = this.$data.formdata.input

                if(data === ""){
                    return
                }

                this.$emit("console-data-send", data)

                //curl -H "Content-Type: application/json" --data '{"command":"/list"}' --request POST http://127.0.0.1:8000/api/execute_cmd
                await fetch("http://127.0.0.1:8000/api/execute_cmd", {
                    method: 'POST',
                    cache: 'no-cache',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ command: data })
                })

                this.$data.formdata.input = ""
            },
        }
    }
</script>

<style scoped>
    .main {
        @apply flex h-[5hv] max-w-[95%] mt-1 ml-auto mr-auto mb-auto justify-center items-center border-solid border-2 border-zinc-800 bg-zinc-400
    }

    .input {
        @apply ml-1 outline-none float-left w-[93%] h-9 bg-zinc-400
    }
    .button {
        @apply border-none float-right ml-auto mr-2 max-w-[7%] h-[3vh]
    }
</style>