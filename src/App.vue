<script setup lang="ts">
import { useAuth } from "./composables/useAuth";

useAuth().initAuth()
</script>

<template>
  <router-view />
</template>
