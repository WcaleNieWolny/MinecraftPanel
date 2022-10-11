<template>
    <div class="rounded-xl m-auto w-[92%] min-w-[92%] h-[3vh] min-h-[3vh] bg-zinc-800 shadow-sm" @keydown="onKeydown">
        <input v-model="formdata" class="outline-none rounded-bl-xl rounded-tl-xl float-left pl-1 w-[calc(100%-42px)] h-full bg-zinc-800 placeholder-zinc-500 text-zinc-300" placeholder="minecraft command"/>
        <div class="hover:bg-green-700 hover:shadow-xl transition transform rounded-br-xl rounded-tr-xl float-right h-[3vh] min-h-[3vh]">
            <button @click="onClick()" class="w-[38px] text-center h-full"><svg class="m-auto pr-2" height="calc(3vh * 0.85)" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="m23.968 0-23.968 10.286h13.68v13.714z"/></svg></button>
        </div>
    </div>
</template>

<script setup lang="ts">

    const formdata = ref("")
    let history = [] as string[]
    let historyIndex = -1

    const emit = defineEmits<{
        (e: 'console-data-send', id: String): void
    }>()

    const onClick = async ()  => {

        const data = formdata.value;

        if(data === ""){
            return
        }
     
        emit('console-data-send', data)

        await fetch("http://127.0.0.1:8000/api/execute_cmd", {    
            method: 'POST',
            cache: 'no-cache',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ command: data })
        })

        history.push(data)
        formdata.value = ""
        historyIndex = history.length - 1

    }

    async function handleHistory(index: number, history: string[]){
        var command = history[index]
        if(formdata.value !== command){
            formdata.value = command
        }
    }

    async function onKeydown(event: KeyboardEvent) {
        switch (event.key){
            case "Enter":
                await onClick();
                break;
            case "ArrowUp":
                if(history.length === 0){
                    return
                }

                event.preventDefault() 
                
                var index;
                if(historyIndex === -1){
                    index = history.length - 1
                    if(history.length != 1) {
                        historyIndex = index
                    }
                }else{
                    index = historyIndex
                    if(index > 0){
                        index--
                        historyIndex = index
                    }
                }

                await handleHistory(index, history)
                break
            case "ArrowDown":
                if(history.length === 0){
                    return
                }

                event.preventDefault()
                
                var index //did not work when inline declaring var (?)
                index = historyIndex

                if(history.length > index + 1 || history.length === index){
                    index++
                    historyIndex = index
                }

                await handleHistory(index, history)
                break
        }
    }
</script>