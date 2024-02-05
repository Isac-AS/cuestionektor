<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { ref, inject } from "vue";
import icons from '../assets/icons/'
import { CreateNotification } from "../services/notifications.service";
import { parsePdf } from "../services/questionnaire.service";
import { LoadQuestionnaires } from "../services/context.service";
import { InformAboutResult } from "../App.vue";
import { CREATE_NOTIFICATION_KEY, INFORM_ABOUT_RESULT_KEY, REFRESH_QUESTIONNAIRES_KEY } from "../injectionKeys";

const createNotification = <CreateNotification>inject(CREATE_NOTIFICATION_KEY);
const refreshQuestionnaires = inject<LoadQuestionnaires>(REFRESH_QUESTIONNAIRES_KEY);
const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);

let uploadedPdfFile: any = null;
const pdfFileName = ref('');
const isPdfUploaded = ref(false);
const isProcessingPdf = ref(false);
const pdfTab = ref(true);

async function handlePdfChange() {
    uploadedPdfFile = await open({
        multiple: false,
        title: "Selecciona el pdf",
        filters: [{
            name: '*',
            extensions: ['pdf']
        }]
    })

    if (uploadedPdfFile != null) {
        isPdfUploaded.value = true;
        createNotification({
            type: 'success',
            message: 'Fichero subido correctamente',
            duration: 3,
        })
    }
}

async function uploadPdf() {
    if (!uploadedPdfFile) {
        createNotification({
            type: 'error',
            message: 'Sube un pdf primero',
            duration: 3,
        });
        return;
    }
    if (pdfFileName.value.length <= 0) {
        createNotification({
            type: 'error',
            message: 'Dale un nombre al cuestionario',
            duration: 3,
        });
        return;
    }

    parsePdf(uploadedPdfFile, pdfFileName.value).then(
        (uploadResult) => {
            informAboutResult!(
                uploadResult.result,
                "Documento procesado correctamente.",
                "Error al procesar el pdf.",
            );
            pdfFileName.value = '';
            uploadedPdfFile = null;
            isProcessingPdf.value = false;
            refreshQuestionnaires!();
        }
    );
}

async function handleTxtUpload() {
    createNotification({
        type: 'error',
        message: 'Todavia no implementado',
        duration: 3,
    });
}
</script>

<template>
    <div class="flex flex-col mt-7 items-center w-full">
        <div class="flex w-2/5">
            <button @click="pdfTab = true" :class="`${pdfTab ? 'bg-wm-primary/80 dark:bg-primary/80' : ''}`"
                class="w-1/2 flex justify-center mx-1 p-2 bg-wm-primary/30 dark:bg-primary/30 rounded hover:brightness-110 transition-all duration-300">
                <p class="font-semibold">Subir un pdf</p>
            </button>
            <button @click="pdfTab = false" :class="`${!pdfTab ? 'bg-wm-primary/80 dark:bg-primary/80' : ''}`"
                class="w-1/2 flex justify-center mx-1 p-2 bg-wm-primary/30 dark:bg-primary/30 rounded hover:brightness-110 transition-all duration-300">
                <p class="font-semibold">Subir un fichero de texto</p>
            </button>
        </div>
        <hr class="h-px mt-2 mb-6 bg-gray-500 border-0 w-2/5">
        <Transition enter-from-class="-translate-x-[150%] opacity-0" leave-to-class="-translate-x-[150%] opacity-0"
            enter-active-class="transition duration-1000" leave-active-class="transition duration-200">
            <div v-if="pdfTab" class="flex flex-col dark:bg-surface-dp6 rounded shadow-md items-center p-4 gap-5 w-1/2">
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
                <div class="flex flex-col gap-5 mt-7" v-if="!isProcessingPdf">
                    <button @click="handlePdfChange" :class="`${isPdfUploaded ? 'btn-secondary' : 'btn-primary'}`"
                        class="flex justify-center transition-all duration-200">
                        <img :src="icons.file" class="mr-2 w-6">
                        <p v-if="!isPdfUploaded">Abrir fichero</p>
                        <p v-else>Abrir otro fichero</p>
                    </button>
                    <input type="text" placeholder="Nombre del cuestionario" v-model="pdfFileName"
                        class="rounded shadow h-7 pl-5 text-OnPrimary focus:ring-3 focus:ring-sky-500">
                    <button @click="isProcessingPdf = true" @click.prevent="uploadPdf"
                        class="btn-primary flex justify-center" :disabled="!isPdfUploaded || pdfFileName.length <= 0">
                        <img :src="icons.settings" class="mr-2 w-6">
                        Procesar cuestionario
                    </button>
                </div>
                <div v-else class="flex p-3 rounded shadow-lg bg-surface-dp24 my-6">
                    <img :src="icons.settings" class="mr-2 w-6 animate-spin invert">
                    <h1 class="text-2xl font-semibold">Procesando...</h1>
                </div>
            </div>
        </Transition>
        <Transition enter-from-class="translate-y-[150%] opacity-0" leave-to-class="translate-y-[150%] opacity-0"
            enter-active-class="transition duration-1000" leave-active-class="transition duration-200">
            <div v-if="!pdfTab" class="flex flex-col dark:bg-surface-dp6 rounded shadow-md items-center p-4 gap-5 w-1/2">
                <h3 class="text-2xl mt-5 font-semibold">Subir un fichero de texto</h3>
                <p>Convierte el pdf u otro tipo de fichero a texto por internet.</p>
                <p>Por defecto se procesa de la misma manera que el pdf.</p>
                <p>Adicionalmente se da la opcion de usar expresiones regulares:</p>

                <div class="flex flex-col gap-5 mt-7">
                    <button @click="handlePdfChange" class="btn-primary flex justify-center">
                        Abrir fichero
                    </button>
                    <input type="text" placeholder="Nombre del cuestionario" v-model="pdfFileName"
                        class="rounded shadow h-7 pl-5 text-OnPrimary focus:ring-3 focus:ring-sky-500">
                    <button @click="handleTxtUpload" class="btn-primary flex justify-center"
                        :disabled="!uploadedPdfFile || pdfFileName.length <= 0">
                        Procesar cuestionario
                    </button>
                </div>
            </div>
        </Transition>
    </div>
</template>
