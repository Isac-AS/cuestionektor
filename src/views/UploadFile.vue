<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { ref } from "vue";
import { parsePdf } from "../services/endpoints.service";

let uploaded_pdf_file: any = null;
const pdf_file_name = ref('');

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
    <div class="page column-nowrap">
        <header>
            <h1>
                Sube un fichero
            </h1>
        </header>
        <div class="column-nowrap jcc aic gap w-100">
            <div class="row-nowrap w-100 card-container">
                <div class="column-nowrap card">
                    <h3>Subir un pdf</h3>
                    <p class="w-100">Experimental. Funciona para cuestionarios similares a los que aparecen en el siguiente
                        enlace:</p>
                    <a
                        href="https://www3.gobiernodecanarias.org/sanidad/scs/contenidoGenerico.jsp?idCarpeta=ef9d69bb-4e38-11ee-ad2e-999944865aa9&idDocument=1d13e463-4e35-11ee-ad2e-999944865aa9">
                        Preguntas scs
                    </a>
                    <p>Se convierte el pdf a texto y se buscan:</p>
                    <ul>
                        <li>
                            Preguntas que empiecen por un numero seguido de un punto.
                            <ul>
                                <li>
                                    Ejemplo: "<b>593.</b> Resto de la pregunta..."
                                </li>
                                <li>
                                    Expresion regular: <b>r"^\d{1,3}\."</b>
                                </li>
                            </ul>
                        </li>
                        <li>
                            Opciones que empiecen por una leta seguida de un parentesis.
                            <ul>
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
                            <ul>
                                <li>
                                    Ejemplo: "<b>Respuesta Correcta</b>: A"
                                </li>
                                <li>
                                    Expresion regular: <b>r"^Respuesta Correcta"</b>
                                </li>
                            </ul>
                        </li>
                    </ul>
                    <div class="gap column">
                        <button @click="handlePdfChange">Abrir fichero</button>
                        <input type="text" placeholder="Nombre del cuestionario" class="w-100" v-model="pdf_file_name">
                        <button @click="uploadPdf">Procesar cuestionario</button>
                    </div>
                </div>
                <div class="column-nowrap card">
                    <h3>Subir un fichero de texto</h3>
                    <p>Convierte el pdf u otro tipo de fichero a texto por internet.</p>
                    <p>Por defecto se procesa de la misma manera que el pdf.</p>
                    <p>Adicionalmente se da la opcion de usar expresiones regulares:</p>

                    <input ref="file" v-on:change="handleTxtUpload()" type="file" class="button">
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.card-container {
    width: 100%;
    justify-content: space-around;
}

.p {
    text-align: justify;
}

input {
    cursor: pointer;
}
</style>