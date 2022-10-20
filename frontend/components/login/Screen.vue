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
        
        <div class="mb-auto ml-auto mr-auto mt-6">
            <div class="h-[24px]">
                <Transition>
                    <div v-if="showError" class="text-center w-[18rem] shadow-inner pl-5 pr-5 mb-auto ml-auto mr-auto bg-zinc-800 text-white border-solid rounded-md delay-75">
                        <h3>{{ errorText }}</h3>
                        <!-- Invalid username or password! -->
                    </div>
                </Transition>
            </div>
        </div>
        
    </div>
</template>

<script setup lang="ts">
    const runtimeConfig = useRuntimeConfig()
    var username = ref("")
    var password = ref("")


    const auth = useState('auth', () => false)

    var showError = ref(false)
    var errorText = ref("Checking credentials...")

    const submitAuthForm = async () => {
        const pwd = password.value
        const name = username.value

        if(pwd.length == 0 || name.length == 0){
            return
        }
        
        showError.value = true;

        let t0 = performance.now();
        let response = await $fetch(`${runtimeConfig.public.apiUrl}/auth/authenticate_user`, {    
            method: 'POST',
            cache: 'no-cache',
            credentials: 'include',
            body: { username: name, password: pwd }
        })
        let t1= performance.now();

        if(response.status === 400){
            let took = (t1-t0);
            if (took < 400) {
                await new Promise(resolve => setTimeout(resolve, 400 - took))
            }
            errorText.value = "Invalid username or password!"
        }else{
            auth.value = true
            navigateTo("console/")
        }
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
}

/* we will explain what these classes do next! */
.v-enter-active,
.v-leave-active {
  transition: opacity 0.4s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>