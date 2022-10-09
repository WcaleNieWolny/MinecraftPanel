<template>
    <div class="m-auto w-[92%] min-w-[92%] h-[3vh] min-h-[3vh] bg-zinc-800">
        <input class="outline-none float-left w-[calc(100%-42px)] h-full bg-zinc-800 placeholder-zinc-500 text-zinc-300" placeholder="minecraft command"/>
        <div class="hover:bg-green-700 hover:shadow-xl border-none float-right h-[3vh] min-h-[3vh]">
            <button class="w-[38px] text-center mr-1 mt-[-0.03rem]"><svg class="m-auto" height="3vh" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="m23.968 0-23.968 10.286h13.68v13.714z"/></svg></button>
        </div>
    </div>
</template>

<!-- <script lang="ts">
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
</script> -->