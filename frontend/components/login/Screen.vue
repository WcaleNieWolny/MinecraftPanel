<template>
    <div class="flex h-screen flex-col">
        <!-- w-[75%] h-4/5 -->
        <div class="bg-zinc-800 m-auto mb-0 p-10 border-0 border-cyan-700 border-solid rounded-lg shadow-xxl">
            <h1 class=" text-5xl bg-gradient-to-r from-sky-600 to-indigo-600 inline-block text-transparent bg-clip-text text-center">Minecraft Admin Panel</h1>
            <br>
            <div class="content-center flex justify-center flex-col mt-4">
                <form class="text-center" @submit.prevent="submitAuthForm">
                    <label class="text-white" for="fname">Username:</label><br>
                    <input type="text" id="username" name="username" v-model="username" class="h-8 bg-zinc-600 border-0 border-indigo-600 border-solid rounded-md w-64"><br><br>
                    <label class="text-white" for="pwd">Password:</label><br>
                    <input type="password" id="pwd" name="pwd" v-model="password" class="h-8 bg-zinc-600 border-0 border-indigo-600 border-solid rounded-md w-64"><br>
                    <br>
                    <input class="text-white border-2 bg-gradient-to-r from-sky-600 to-indigo-600 border-cyan-700 border-solid rounded-md w-24 hover:scale-110 duration-100" type="submit" value="Submit">
                </form> 
            </div>
        </div>
        <!-- bg-gradient-to-r from-sky-800 to-indigo-800 -->
        <div :style="{visibility: showError ? 'visible' : 'hidden'}" class="text-center w-[18rem] shadow-inner pl-5 pr-5 mb-auto ml-auto mr-auto mt-6 bg-zinc-800 text-white border-solid rounded-md delay-75">
            <h3>Invalid username or password!</h3>
        </div>
    </div>
</template>

<script setup lang="ts">
    const apiUrl = useApiUrl()
    var username = ref("")
    var password = ref("")

    var showError = ref(false)

    const submitAuthForm = async () => {
        const pwd = password.value
        const name = username.value

        if(pwd.length == 0 || name.length == 0){
            return
        }
        
        let response = await fetch(`${apiUrl.value}/auth/authenticate_user`, {    
            method: 'POST',
            cache: 'no-cache',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username: name, password: pwd })
        })

        if(response.status === 400){
            showError.value = true;
        }

        await fetch(`${apiUrl.value}/auth/test`, {
            method: 'GET',
            credentials: 'include'
        })
    }
</script>

<style>
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer components {
  .shadow-xxl {
    box-shadow: rgba(0, 0, 0, 0.25) 0px 54px 55px, rgba(0, 0, 0, 0.11) 0px -12px 30px, rgba(0, 0, 0, 0.11) 0px 4px 6px, rgba(0, 0, 0, 0.17) 0px 12px 13px, rgba(0, 0, 0, 0.09) 0px -3px 5px;
  }
  /* ... */
}
</style>