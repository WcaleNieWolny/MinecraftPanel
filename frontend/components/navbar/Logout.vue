<template>
    <button class="text-zinc-300" @click="onClick">
        <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-logout" width="3vh" height="3vh" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
            <path d="M14 8v-2a2 2 0 0 0 -2 -2h-7a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h7a2 2 0 0 0 2 -2v-2"></path>
            <path d="M7 12h14l-3 -3m0 6l3 -3"></path>
        </svg>
    </button>
</template>

<script setup lang="ts">
    const runtimeConfig = useRuntimeConfig()
    const auth = useState('auth')

    async function onClick() { 
        auth.value = false

        await $fetch(`${runtimeConfig.public.apiUrl}/auth/logout`, {    
            method: 'GET',
            cache: 'no-cache',
            credentials: 'include',
        })

        navigateTo("/")
    }
</script>