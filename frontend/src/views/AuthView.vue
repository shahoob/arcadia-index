<template>
  <div class="auth-form">
    <LoginForm v-if="route.path.includes('login')" />
    <RegisterForm v-if="route.path.includes('register')" />
  </div>
</template>
<script lang="ts" setup>
import LoginForm from '@/components/auth/LoginForm.vue'
import RegisterForm from '@/components/auth/RegisterForm.vue'
import { useRoute } from 'vue-router'

const route = useRoute()
</script>

<style>
.auth-form {
  align-items: center;
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.form-item {
  margin-top: 10px;
}
</style>
