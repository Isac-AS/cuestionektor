import { ref } from "vue";
import { Questionnaire } from "../models/questionnaire";
import { getQuestionnaires } from "./questionnaire.service";

export type LoadQuestionnaires = { (): Promise<void> };
export type SetCurrentQuestionnaireId = { (id: number): void };
export default function useContext() {
    const registeredQuestionnaires = ref<Questionnaire[]>();
    const loadQuestionnaires: LoadQuestionnaires = async () => {
        let questionnaireResponse = await getQuestionnaires();
        registeredQuestionnaires.value = questionnaireResponse.data.sort(
            (a, b) => (a.last_accessed < b.last_accessed) ? 1 : (b.last_accessed < a.last_accessed) ? -1 : 0
        );
    };

    const currentQuestionnaireId = ref();
    const setCurrentQuestionnaireId: SetCurrentQuestionnaireId = (id: number) => {
        currentQuestionnaireId.value = id;
    }

    return {
        registeredQuestionnaires,
        loadQuestionnaires,
        currentQuestionnaireId,
        setCurrentQuestionnaireId
    }
}