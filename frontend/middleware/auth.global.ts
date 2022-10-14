export default defineNuxtRouteMiddleware((to, from) => {
    const auth = useState('auth');

    if((auth.value === undefined || auth.value === false) && to.fullPath != "/"){
        return navigateTo('/');
    }
  }) 