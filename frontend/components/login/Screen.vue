<template>
    <div class="flex h-screen">
        <!-- w-[75%] h-4/5 -->
        <div class="bg-zinc-800 m-auto p-10 border-2 border-cyan-700 border-solid rounded-lg">
            <h1 class=" text-5xl text-cyan-700 text-center">Minecraft Admin Panel</h1>
            <br>
            <div class="content-center flex justify-center flex-col">
                <form class="text-center" @submit.prevent="submitAuthForm">
                    <label class="text-white" for="fname">Username:</label><br>
                    <input type="text" id="username" name="username" v-model="username" class="h-8 bg-zinc-600 border-2 border-cyan-700 border-solid rounded-md w-64"><br><br>
                    <label class="text-white" for="pwd">Password:</label><br>
                    <input type="password" id="pwd" name="pwd" v-model="password" class="h-8 bg-zinc-600 border-2 border-cyan-700 border-solid rounded-md w-64"><br>
                    <br>
                    <input class="text-white border-2 bg-zinc-600 border-cyan-700 border-solid rounded-md w-16" type="submit" value="Submit">
                </form> 
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    const apiUrl = useApiUrl()

    var username = ref("")
    var password = ref("")

    const submitAuthForm = async () => {

        const pwd = username.value
        const name = password.value

        if(password.length == 0 || username.length == 0){
            return
        }

    }

</script>