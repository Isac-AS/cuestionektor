import { ref } from "vue";
import { Questionnaire } from "../models/questionnaire";
import { getQuestionnaires } from "./questionnaire.service";

export type LoadQuestionnaires = {(): Promise<void>};
export default function useContext() {
    const registeredQuestionnaires = ref<Questionnaire[]>();
    const loadQuestionnaires: LoadQuestionnaires = async () => {
        let questionnaireResponse = await getQuestionnaires();
        registeredQuestionnaires.value = questionnaireResponse.data;
    };
    
    return {
        registeredQuestionnaires,
        loadQuestionnaires
    }
}