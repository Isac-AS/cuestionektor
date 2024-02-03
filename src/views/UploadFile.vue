<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { ref } from "vue";
import { parsePdf } from "../services/endpoints.service";

let uploaded_pdf_file: any = null;
const pdf_file_name = ref('');
const pdf_tab = ref(true);

async function handlePdfChange() {
    uploaded_pdf_file = await open({
        multiple: false,
        title: "Selecciona el pdf",
        filters: [{
            name: '*',
            extensions: ['pdf']
        }]
    })
    console.log(uploaded_pdf_file)
}

async function uploadPdf() {
    if (!uploaded_pdf_file) {
        alert("Sube un pdf primero");
        return;
    }
    if (pdf_file_name.value.length <= 0) {
        alert("Dale un nombre al cuestionario");
        return;
    }

    let result: boolean = await parsePdf(uploaded_pdf_file, pdf_file_name.value);
    console.log(result)
}

async function handleTxtUpload() {
    alert("Todavia no implementado.")
}
</script>

<template>
    <div class="flex flex-col mt-7 items-center w-full">
        <div class="flex w-2/5">
            <button @click="pdf_tab = true" :class="`${pdf_tab ? 'bg-wm-primary/80 dark:bg-primary/80' : ''}`"
                class="w-1/2 flex justify-center mx-1 p-2 bg-wm-primary/30 dark:bg-primary/30 rounded hover:brightness-110 transition-all duration-300">
                <p class="font-semibold">Subir un pdf</p>
            </button>
            <button @click="pdf_tab = false" :class="`${!pdf_tab ? 'bg-wm-primary/80 dark:bg-primary/80' : ''}`"
                class="w-1/2 flex justify-center mx-1 p-2 bg-wm-primary/30 dark:bg-primary/30 rounded hover:brightness-110 transition-all duration-300">
                <p class="font-semibold">Subir un fichero de texto</p>
            </button>
        </div>
        <hr class="h-px mt-2 mb-6 bg-gray-500 border-0 w-2/5">
        <Transition enter-from-class="-translate-x-[150%] opacity-0" leave-to-class="-translate-x-[150%] opacity-0"
            enter-active-class="transition duration-1000" leave-active-class="transition duration-200">
            <div v-if="pdf_tab" class="flex flex-col dark:bg-surface-dp6 rounded shadow-md items-center p-4 gap-5 w-1/2">
                <h3 class="text-2xl mt-5 font-semibold">Subir un pdf</h3>
                <p class="w-4/5">
                    Funciona <b>solo</b> para cuestionarios similares a los que aparecen en el siguiente enlace:
                </p>
                <a class="btn-secondary font-semibold"
                    href="https://www3.gobiernodecanarias.org/sanidad/scs/contenidoGenerico.jsp?idCarpeta=ef9d69bb-4e38-11ee-ad2e-999944865aa9&idDocument=1d13e463-4e35-11ee-ad2e-999944865aa9">
                    Preguntas SCS
                </a>
                <p class="text-lg mt-6">Se convierte el pdf a texto y se buscan:</p>
                <ul class="list-decimal">
                    <li>
                        Preguntas que empiecen por un numero seguido de un punto.
                        <ul class="list-disc list-inside">
                            <li>
                                Ejemplo: "<b>593.</b> Resto de la pregunta..."
                            </li>
                            <li>
                                Expresion regular: <b>r"^\d{1,3}\."</b>
                            </li>
                        </ul>
                    </li>
                    <li>
                        <p>Opciones que empiecen por una leta seguida de un parentesis.</p>
                        <ul class="list-disc list-inside">
                            <li>
                                Ejemplo: "<b>A) </b> Resto de la respuesta..."
                            </li>
                            <li>
                                Expresion regular: <b>r"^(A|B|C|D)\)"</b>
                            </li>
                        </ul>
                    </li>
                    <li>
                        Respuestas que empiecen por la ristra "Respuesta Correcta"
                        <ul class="list-disc list-inside">
                            <li>
                                Ejemplo: "<b>Respuesta Correcta</b>: A"
                            </li>
                            <li>
                                Expresion regular: <b>r"^Respuesta Correcta"</b>
                            </li>
                        </ul>
                    </li>
                </ul>
                <div class="flex flex-col gap-5 mt-7">
                    <button @click="handlePdfChange" class="btn-primary flex justify-center">
                        Abrir fichero
                    </button>
                    <input type="text" placeholder="Nombre del cuestionario" v-model="pdf_file_name"
                        class="rounded shadow h-7 pl-5 text-OnPrimary focus:ring-3 focus:ring-sky-500">
                    <button @click="uploadPdf" class="btn-primary flex justify-center"
                        :disabled="!uploaded_pdf_file || pdf_file_name.length <= 0">
                        Procesar cuestionario
                    </button>
                </div>
            </div>
        </Transition>
        <Transition enter-from-class="translate-y-[150%] opacity-0" leave-to-class="translate-y-[150%] opacity-0"
            enter-active-class="transition duration-1000" leave-active-class="transition duration-200">
            <div v-if="!pdf_tab" class="flex flex-col dark:bg-surface-dp6 rounded shadow-md items-center p-4 gap-5 w-1/2">
                <h3 class="text-2xl mt-5 font-semibold">Subir un fichero de texto</h3>
                <p>Convierte el pdf u otro tipo de fichero a texto por internet.</p>
                <p>Por defecto se procesa de la misma manera que el pdf.</p>
                <p>Adicionalmente se da la opcion de usar expresiones regulares:</p>

                <div class="flex flex-col gap-5 mt-7">
                    <button @click="handlePdfChange" class="btn-primary flex justify-center">
                        Abrir fichero
                    </button>
                    <input type="text" placeholder="Nombre del cuestionario" v-model="pdf_file_name"
                        class="rounded shadow h-7 pl-5 text-OnPrimary focus:ring-3 focus:ring-sky-500">
                    <button @click="handleTxtUpload" class="btn-primary flex justify-center"
                        :disabled="!uploaded_pdf_file || pdf_file_name.length <= 0">
                        Procesar cuestionario
                    </button>
                </div>
            </div>
        </Transition>
    </div>
</template>
