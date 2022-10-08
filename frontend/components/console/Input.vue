<template>
    <div class="shadow-lg flex h-[5hv] max-w-[95%] mt-4 ml-auto mr-auto mb-auto items-center rounded-lg bg-zinc-800" @keydown="onKeydown">
        <input class="ml-1 outline-none float-left w-[100%] h-9 bg-zinc-800 placeholder-zinc-500 text-zinc-300" v-model="formdata.input" placeholder="minecraft command"/>
        <div class="hover:bg-green-700 hover:shadow-xl border-none float-right h-[5hv] pt-0 pl-1">
            <button @click="onClick()"><svg class="relative top-1 bottom-1 right-1" height="34px" width="34px" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="m23.968 0-23.968 10.286h13.68v13.714z"/></svg></button>
        </div>
    </div>
</template>

<script lang="ts">
    export default{
        name: 'Input',

        data() {
            return {
                formdata: { input: ''},
                history: [] as string[],
                historyIndex: -1
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
                    credentials: 'include',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ command: data })
                })

                this.$data.history.push(data)
                this.$data.formdata.input = ""
            },

            async handleHistory(index: number, history: string[]){
                

                var command = history[index]
                if(this.$data.formdata.input !== command){
                    this.$data.formdata.input = command
                }
            },

            async onKeydown(event: KeyboardEvent) {
                switch (event.key){
                    case "Enter":
                        await this.onClick();
                        break;
                    case "ArrowUp":
                        var history = this.$data.history;
                        if(history.length === 0){
                            return
                        }

                        event.preventDefault() 
                        
                        var index;
                        if(this.$data.historyIndex === -1){
                            index = history.length - 1
                            this.$data.historyIndex = index
                        }else{
                            index = this.$data.historyIndex
                            if(index > 0){
                                index--
                                this.$data.historyIndex = index
                            }
                        }

                        await this.handleHistory(index, history)
                        break
                    case "ArrowDown":
                        var history = this.$data.history;
                        if(history.length === 0){
                            return
                        }

                        event.preventDefault()
                        
                        var index //did not work when inline declaring var (?)
                        index = this.$data.historyIndex

                        if(history.length > index + 1 || history.length === index){
                            index++
                            this.$data.historyIndex = index
                        }

                        await this.handleHistory(index, history)
                        break
                }
            },
        }
    }
</script>