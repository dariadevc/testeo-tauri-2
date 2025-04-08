<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const anio_nacimiento = ref<number | null>(null)
const edad = ref<number | null>(null)

async function calcular_edad() {
    if (anio_nacimiento.value === null) {
        edad.value = null
        return
    }

    try {
    console.log("Invocando cálculo de edad...");

    if (!anio_nacimiento.value || isNaN(anio_nacimiento.value)) {
        edad.value = null;
        return;
    }

    console.log("Enviando a Rust:", { anio_nacimiento: anio_nacimiento.value });

    
    const resultado = await invoke<number>("calcular_edad", {
        anio_nacimiento: anio_nacimiento.value,
    });

    console.log("Resultado recibido:", resultado);
    edad.value = resultado;

    } catch (error) {
    console.error("Error al calcular la edad:", error);
    edad.value = null;
    }
}
</script>

<template>
    <div>
        <h1>Calculadora de edad</h1>
        <form @submit.prevent="calcular_edad">
            <input v-model.number="anio_nacimiento" type="number" placeholder="Ingresá tu año de nacimiento">
            <button type="submit">Calcular Edad</button>
        </form>
        <p v-if="edad !== null">{{ edad }}</p>
    </div>
</template>